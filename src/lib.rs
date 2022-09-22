extern crate log;

mod requests;
mod responses;
use requests::{CountriesRequest, Endpoint, HolidaysRequest};
use std::{collections::HashMap, error::Error};

use regex::Regex;
use reqwest::{Response, Url};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct HolidayAPI {
    base_url: String,
    key: String,
}

#[derive(Debug)]
pub enum HolidayAPIError {
    InvalidKey,
    InvalidVersion(String),
}

#[derive(strum_macros::Display, Debug, Clone, Serialize, Deserialize)]
pub enum Format {
    CSV,
    JSON,
    PHP,
    TSV,
    YAML,
    XML,
}

impl HolidayAPI {
    fn is_valid_key(key: &str) -> bool {
        let uuid_regex =
            Regex::new(r"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}")
                .expect("Regex is correct");

        uuid_regex.is_match(key)
    }

    fn is_valid_version(version: i32) -> Option<String> {
        let valid_versions = [1];
        if !valid_versions.contains(&version) {
            Some(format!("{} is not a valid version.\n", version))
        } else {
            None
        }
    }
    fn construct_api(key: &str, version: i32) -> HolidayAPI {
        HolidayAPI {
            base_url: format!("https://holidayapi.com/v{}/", version),
            key: key.to_owned(),
        }
    }

    pub fn new(key: &str) -> Result<HolidayAPI, HolidayAPIError> {
        if !Self::is_valid_key(key) {
            return Err(HolidayAPIError::InvalidKey);
        }
        Ok(Self::construct_api(key, 1))
    }

    pub fn with_version(key: &str, version: i32) -> Result<HolidayAPI, HolidayAPIError> {
        if !Self::is_valid_key(key) {
            return Err(HolidayAPIError::InvalidKey);
        }
        if let Some(error) = Self::is_valid_version(version) {
            return Err(HolidayAPIError::InvalidVersion(error));
        }
        Ok(Self::construct_api(key, version))
    }

    async fn request(
        &self,
        endpoint: Endpoint,
        parameters: HashMap<String, String>,
    ) -> Result<Response, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let url = Url::parse(self.base_url.as_str()).unwrap();

        let url = url
            .join(endpoint.to_string().to_ascii_lowercase().as_str())
            .unwrap();

        let url = Url::parse_with_params(&format!("{}?key={}", url, self.key), parameters).unwrap();

        let response = client.get(url).send().await?;
        Ok(response)
    }

    pub fn countries(&self) -> CountriesRequest {
        CountriesRequest::new(self)
    }

    pub fn holidays(&self, country: String, year: i32) -> HolidaysRequest {
        HolidaysRequest::new(self, country, year)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    static EXPIRED_KEY: &str = "a112a6bb-a47c-4aa7-b1d2-aaaab24aaacf";

    #[tokio::test]
    async fn test_countries_api() {
        let api = HolidayAPI::new(EXPIRED_KEY).unwrap();
        let response = api
            .countries()
            .country("us")
            .public(true)
            .get()
            .await
            .unwrap();
        println!("{:?}", response);
    }
}
