use reqwest::{Response, header};
use scraper::{Html, Selector};
use google_maps::{GoogleMapsClient, prelude::*};
use std::{error::Error, env};

fn parse_response(response : &Response)
{
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
    let departure_time = DepartureTime::At(NaiveDate::from_ymd(2023, 2, 9).and_hms(8, 30, 0));
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
    println!("Directions: {:#?}", directions);
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
    let gmaps_api_key: String = String::from(env!("GOOGLE_MAPS_API_KEY"));
    let gmaps_client = GoogleMapsClient::new(gmaps_api_key.as_str());
    let url = "https://www.utahrealestate.com/1849266?st_id=182956172&actor=88145";
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
