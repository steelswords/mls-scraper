use reqwest::{Response, Error, header};

fn parse_response(response : &Response)
{

}
// Some help from https://www.scrapingbee.com/blog/web-scraping-rust/
fn main() {
    //let response = reqwest::blocking::get( "https://www.utahrealestate.com/1849266?st_id=182956172&actor=88145")
    let mut url = "https://google.com";
    url = "https://www.utahrealestate.com/1849266?st_id=182956172&actor=88145";
    //url = "https://www.utahrealestate.com";
    //url = "http://127.0.0.1:8088/123";
    //let response = reqwest::blocking::get(url);
    let mut headers = header::HeaderMap::new();
    headers.insert(header::USER_AGENT, header::HeaderValue::from_static("rust/1.0"));
    let client = reqwest::blocking::Client::builder()
        .user_agent("rust/1.0")
        .default_headers(headers)
        .gzip(true)
        .build().unwrap();
    let response = client.get(url).send();
    match response {
        Ok(r) => {
            print!("Got a response (code {}): ", &r.status());
            let text: String = r.text().unwrap().clone();
            println!("{}", text);
            let document = scraper::Html::parse_document(text.as_str());
            //let mls_selector = scraper::Selector::parse("span.facts-header").unwrap();
            let mls_selector = scraper::Selector::parse("div.facts___item>div").unwrap();
            let mls_list = document.select(&mls_selector).map(|x| x.inner_html());
            mls_list.for_each(|item| println!("MLS: {}", item));
        },
        Err(e) => eprint!("Got an error: {}", e)
    }

    //let client = reqwest::Client::new();
    //let response = client.get(url);
    //println!("{}", response.await.unwrap().text().await);

    /*
    let response = reqwest::blocking::get(url)
    .unwrap()
    .text()
    .unwrap();
    */

    //println!("Response: {}", response);

    
/*
    let document = scraper::Html::parse_document(&response);

    let title_selector = scraper::Selector::parse("h3>a").unwrap();

    let titles = document.select(&title_selector).map(|x| x.inner_html());

    titles
        .for_each(|item| println!("MLS: {}", item));
    */
}
/*
fn main() {
    println!("Hello, world!");
    let url = "https://www.utahrealestate.com/1849266?st_id=182956172&actor=88145";
    let response = reqwest::blocking::get(url).unwrap();
    let response_text = response.text().unwrap();

    println!("Response: {}", response_text);
    let document = scraper::Html::parse_document(&response_text);
    let mls_selector = scraper::Selector::parse("span.facts-header").unwrap();
    let mls_list = document.select(&mls_selector).map(|x| x.inner_html());
    mls_list.for_each(|item| println!("MLS: {}", item));
    println!("Done.");
}


*/
