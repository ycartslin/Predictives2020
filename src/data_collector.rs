use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct Filing {
    case_id: String,
    text: String,
    outcome: String,
}

fn fetch_filings(api_url: &str) -> Result<Vec<Filing>, Box<dyn Error>> {
    let client = Client::new();
    let response = client.get(api_url).send()?;
    let filings = response.json::<Vec<Filing>>()?;
    Ok(filings)
}

pub fn load_filings() -> Result<(), Box<dyn Error>> {
    let filings = fetch_filings("https://api.example.com/filings")?;
    for filing in filings {
        println!("{:?}", filing);
    }
    Ok(())
}
