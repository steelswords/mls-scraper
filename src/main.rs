use reqwest::{Response, Error, header};

fn parse_response(response : &Response)
{

}
// Some help from https://www.scrapingbee.com/blog/web-scraping-rust/
fn main() {
    let url = "https://www.utahrealestate.com/1849266?st_id=182956172&actor=88145";
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
            //print!("Got a response (code {}): ", &r.status());
            let text: String = r.text().unwrap().clone();
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
        },
        Err(e) => eprint!("Got an error: {}", e)
    }
}
