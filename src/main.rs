use reqwest::{Response, header};
use scraper::{Html, Selector};
use google_maps::{GoogleMapsClient, prelude::*};
use std::{error::Error, env};
use dotenv::dotenv;
use chrono::prelude::*;

fn parse_response(response : &Response)
{
}

// Returns the next Tuesday.
// This is to get a representative day for commute times. We don't want Fridays,
// because there is less traffic on those days on I-15, at least (YMMV).
fn get_next_commute_day() -> NaiveDate {
    let now = chrono::Local::today();
    let day_offset = match now.weekday() {
        Weekday::Mon => 1,
        Weekday::Tue => 7,
        Weekday::Wed => 6,
        Weekday::Thu => 5,
        Weekday::Fri => 4,
        Weekday::Sat => 3,
        Weekday::Sun => 2,
    };
    let this_tuesday = now + Duration::days(day_offset);
    NaiveDate::from_ymd(this_tuesday.year(), this_tuesday.month(), this_tuesday.day())
}

async fn get_commute_time(gmaps: &GoogleMapsClient, house_address: String, work_address: String) -> Result<(), google_maps::directions::error::Error> {
    //use async_std::task;
    /*
    let distance_matrix = gmaps.distance_matrix(
        // Origins
        vec![
            Waypoint::Address(house_address)
        ],
        // Destinations
        vec![
            Waypoint::Address(work_address)
        ]
    ).execute().unwrap();
    

    println!("{:#?}", distance_matrix);
    */
    // TODO: Make this always be next Tuesday except for holidays
    let departure_time = DepartureTime::At(get_next_commute_day().and_hms(8, 45, 0));
    let origin = Location::Address(house_address);
    let destination = Location::Address(work_address);
    /*
    let directions = task::block_on(
        gmaps.directions(origin, destination)
            .with_travel_mode(TravelMode::Driving)
            //.with_departure_time(departure_time)
            .execute()
    );
    */
    let directions = gmaps.directions(origin, destination)
            .with_travel_mode(TravelMode::Driving)
            .with_departure_time(departure_time)
            .execute().await?;
    //println!("Directions: {:#?}", directions);
    let commute_time = directions.routes[0].legs[0].duration.text.clone();
    let commute_time_in_traffic = directions.routes[0].legs[0].get_duration_in_traffic_text().unwrap();

    println!("Commute time: {}\nTime in traffic: {}", commute_time, commute_time_in_traffic);

    Ok(())
}

fn get_address(document: &Html) -> String {
    let address_selector = Selector::parse("div.prop___overview").unwrap();
    let address_list = document.select(&address_selector)
        .next()
        .unwrap()
        .text()
        .collect::<Vec<_>>();
    let address: String = address_list.join("\n");
    address.trim().to_string()
}


// Some help from https://www.scrapingbee.com/blog/web-scraping-rust/
#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn Error>> {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <URL of property to scan>", args[0]);
        // TODO: My Rust game has to step it up.
        return Ok(()); // TODO: Make this an Error instead
    }
    //let url = "https://www.utahrealestate.com/1849266?st_id=182956172&actor=88145";
    let url = args[1].as_str();

    // Initialize the Google Maps API
    dotenv().ok();
    let gmaps_api_key: String = String::from(env::var("GOOGLE_MAPS_API_KEY")
                                             .expect("Edit .env to reflect your Google Maps API Key"));
    let gmaps_client = GoogleMapsClient::new(gmaps_api_key.as_str());

    let mut headers = header::HeaderMap::new();
    headers.insert(header::USER_AGENT, header::HeaderValue::from_static("rust/1.0"));
    let client = reqwest::Client::builder()
        .user_agent("rust/1.0")
        .default_headers(headers)
        .gzip(true)
        .build().unwrap();
    let response = client.get(url).send().await?;
    //print!("Got a response (code {}): ", &r.status());
    let text: String = response.text().await?.clone();
    //println!("{}", text);
    let document = scraper::Html::parse_document(text.as_str());
    let mls_selector = scraper::Selector::parse("div.facts___item>div").unwrap();
    let mls_interior_selector = scraper::Selector::parse("span").unwrap();
    let mls_list = document.select(&mls_selector).map(|x| x.inner_html());
    mls_list.for_each(|item| {
        let inner_text = item.split("\n").collect::<Vec<&str>>()[2];
        let category_fragment = scraper::Html::parse_fragment(item.as_str());

        let category_span = category_fragment.select(&mls_interior_selector).next().unwrap();
        let category = category_span.text().collect::<Vec<_>>()[0];
        println!("{}: {}", category.trim(), inner_text.trim());
    });
    let address = get_address(&document);
    let work_address = String::from("3401 Ashton Blvd, Lehi, UT 84043");
    println!("Address: {}", &address);
    get_commute_time(&gmaps_client, address, work_address).await?;
    Ok(())
}
