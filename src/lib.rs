extern crate log;

mod requests;
mod responses;
use requests::{
    CountriesRequest, Endpoint, HolidaysRequest, LanguagesRequest, WorkdayRequest, WorkdaysRequest,
};
use std::{collections::HashMap, error::Error, fmt};

use regex::Regex;
use reqwest::{Response, StatusCode, Url};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct HolidayAPI {
    base_url: String,
    key: String,
}

#[derive(Debug)]
pub enum HolidayAPIError {
    InvalidKeyFormat(String),
    InvalidOrExpiredKey(String),
    InvalidVersion(String),
}

impl fmt::Display for HolidayAPIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidKeyFormat(key) => write!(f, "Invalid key: {}", key),
            Self::InvalidVersion(version) => write!(f, "Invalid version: {}", version),
            Self::InvalidOrExpiredKey(key) => write!(f, "Invalid or expired key: {}", key),
        }
    }
}
impl Error for HolidayAPIError {}

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
            return Err(HolidayAPIError::InvalidKeyFormat(key.into()));
        }
        Ok(Self::construct_api(key, 1))
    }

    pub fn with_version(key: &str, version: i32) -> Result<HolidayAPI, HolidayAPIError> {
        if !Self::is_valid_key(key) {
            return Err(HolidayAPIError::InvalidKeyFormat(key.into()));
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
        let url = Url::parse(self.base_url.as_str())?;
        let url = url.join(endpoint.to_string().to_ascii_lowercase().as_str())?;
        let url = Url::parse_with_params(&format!("{}?key={}", url, self.key), parameters)?;
        let response = client.get(url).send().await?;
        match response.error_for_status() {
            Ok(res) => Ok(res),
            Err(err) => match err.status() {
                Some(StatusCode::UNAUTHORIZED) => Err(Box::new(
                    HolidayAPIError::InvalidOrExpiredKey(self.key.clone()),
                )),
                Some(_) => Err(Box::new(err)),
                None => unreachable!(),
            },
        }
    }
    pub fn countries(&self) -> CountriesRequest {
        CountriesRequest::new(self)
    }

    pub fn holidays(&self, country: &str, year: i32) -> HolidaysRequest {
        HolidaysRequest::new(self, country.into(), year)
    }

    pub fn workday(&self, country: &str, start: &str, days: usize) -> WorkdayRequest {
        WorkdayRequest::new(self, country, start, days)
    }

    pub fn workdays(&self, country: &str, start: &str, days: usize) -> WorkdaysRequest {
        WorkdaysRequest::new(self, country, start, days)
    }

    pub fn languages(&self) -> LanguagesRequest {
        LanguagesRequest::new(self)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    static EXPIRED_KEY: &str = "daaaaaab-aaaa-aaaa-aaaa-2aaaada37e14";

    #[tokio::test]
    #[ignore]
    async fn test_countries_api() {
        let api = HolidayAPI::new(EXPIRED_KEY).unwrap();
        let response = api.countries().country("us").public(true).get().await;
        println!("{:?}", response);
    }

    #[tokio::test]
    async fn test_holidays_api() {
        let api = HolidayAPI::new(EXPIRED_KEY).unwrap();
        let response = api.holidays("jp", 2021).pretty(true).get().await;
        match response {
            Ok(res) => eprintln!("{:?}", res),
            Err(res) => eprintln!("{}", res),
        }
    }
}
