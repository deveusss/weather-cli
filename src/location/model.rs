
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct Location {
    pub lat: String,
    pub lon: String,
}

impl Location {
    pub fn new(latitude: String, longitude: String) -> Self {
        Self { lat:latitude, lon:longitude }
    }

    pub async fn resolve_location(city_name: &str) -> Result<Location, reqwest::Error> {
        let client = reqwest::Client::new();

        let url = format!("https://nominatim.openstreetmap.org/search?q={}&format=json", city_name);
        print!("{} ", url);
        log::debug!(
            "Querying {} location info from {}",
            city_name,
            url
        );

        let response = client
        .get(&url)
        .header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/98.0.4758.102 Safari/537.36")
        .send()
        .await?;
        let location = response
            .json::<Vec<Location>>()
            .await?
            .first()
            .cloned()
            .expect("No Location Found");
      
        Ok(location)
    }
}
