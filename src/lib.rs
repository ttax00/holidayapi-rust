//! # Aspiration
//! Unofficial library for [Holiday API](https://holidayapi.com) written in Rust. This repo implements interface for original HolidayAPI endpoints seen [here](https://holidayapi.com/docs).
//!
//! ## Acknowledgments
//! This project is heavily inspired by [holidayapi-node](https://github.com/holidayapi/holidayapi-node) and [holiday-api-rust](https://github.com/guibranco/holiday-api-rust) repositories.
extern crate log;

mod requests;
mod responses;
use requests::{
    CountriesRequest, Endpoint, HolidaysRequest, LanguagesRequest, WorkdayRequest, WorkdaysRequest,
};
use std::{collections::HashMap, error::Error, fmt};

use regex::Regex;
use reqwest::{Response, StatusCode, Url};

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

///
///
impl HolidayAPI {
    pub fn is_valid_key(key: &str) -> Result<(), HolidayAPIError> {
        let uuid_regex =
            Regex::new(r"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}")
                .expect("Regex is correct");

        if uuid_regex.is_match(key) {
            Ok(())
        } else {
            Err(HolidayAPIError::InvalidKeyFormat(key.into()))
        }
    }

    pub fn is_valid_version(version: &i32) -> Result<(), HolidayAPIError> {
        let valid_versions = [1];
        if !valid_versions.contains(version) {
            Err(HolidayAPIError::InvalidVersion(format!(
                "Invalid version: {}, please choose: {:?}",
                version, valid_versions
            )))
        } else {
            Ok(())
        }
    }
    fn construct_api(key: &str, version: i32) -> HolidayAPI {
        HolidayAPI {
            base_url: format!("https://holidayapi.com/v{}/", version),
            key: key.to_owned(),
        }
    }
    /// Construct a new holiday API
    ///
    /// # Errors
    ///
    /// Will return an `Err` if the given key is not plausibly a valid one.
    ///
    /// # Examples
    ///
    /// Basic usage
    ///
    /// ```
    /// use holidayapi_rust::HolidayAPI;
    ///
    /// let api = HolidayAPI::new("00000000-0000-0000-0000-000000000000").unwrap();
    /// ```
    pub fn new(key: &str) -> Result<HolidayAPI, HolidayAPIError> {
        Self::is_valid_key(key)?;

        Ok(Self::construct_api(key, 1))
    }

    /// Construct a new holiday API
    ///
    /// # Errors
    ///
    /// Will return an `Err` if the given key is not plausibly a valid one. Or the api version is invalid.
    /// Current valid versions: `[1]`
    ///
    /// # Examples
    ///
    /// Basic usage
    ///
    /// ```
    /// use holidayapi_rust::HolidayAPI;
    ///
    /// let api = HolidayAPI::with_version("00000000-0000-0000-0000-000000000000", 1).unwrap();
    /// ```
    pub fn with_version(key: &str, version: i32) -> Result<HolidayAPI, HolidayAPIError> {
        Self::is_valid_key(key)?;
        Self::is_valid_version(&version)?;

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

    /// Generates a minimal `countries` request and returns it.
    ///
    /// # Examples
    ///
    /// Basic usage
    /// ```
    /// use holidayapi_rust::HolidayAPI;
    ///
    ///	let api = HolidayAPI::new("00000000-0000-0000-0000-000000000000").unwrap();
    /// let request = api.countries();
    /// ```
    ///
    /// Adding optional parameters with builder pattern
    /// ```
    /// use holidayapi_rust::HolidayAPI;
    ///
    ///	let api = HolidayAPI::new("00000000-0000-0000-0000-000000000000").unwrap();
    /// let specific_request = api.countries().search("united states").public(true);
    /// ```
    pub fn countries(&self) -> CountriesRequest {
        CountriesRequest::new(self)
    }

    /// Generates a minimal `holidays` request and returns it.
    ///
    /// # Examples
    ///
    /// Basic usage
    /// ```
    /// use holidayapi_rust::HolidayAPI;
    ///
    ///	let api = HolidayAPI::new("00000000-0000-0000-0000-000000000000").unwrap();
    /// let request = api.holidays("us", 2020);
    /// ```
    ///
    /// Adding optional parameters with builder pattern
    /// ```
    /// use holidayapi_rust::HolidayAPI;
    ///
    ///	let api = HolidayAPI::new("00000000-0000-0000-0000-000000000000").unwrap();
    /// let specific_request = api.holidays("us", 2020).month(12).upcoming(true);
    /// ```
    pub fn holidays(&self, country: &str, year: i32) -> HolidaysRequest {
        HolidaysRequest::new(self, country.into(), year)
    }

    /// Generates a minimal `workday` request and returns it.
    ///
    /// # Examples
    ///
    /// Basic usage
    /// ```
    /// use holidayapi_rust::HolidayAPI;
    ///
    ///	let api = HolidayAPI::new("00000000-0000-0000-0000-000000000000").unwrap();
    /// let request = api.workday("us","YYYY-MM-DD", 100);
    /// ```
    pub fn workday(&self, country: &str, start: &str, days: i32) -> WorkdayRequest {
        WorkdayRequest::new(self, country, start, days)
    }

    /// Generates a minimal `workdays` request and returns it.
    ///
    /// # Examples
    ///
    /// Basic usage
    /// ```
    /// use holidayapi_rust::HolidayAPI;
    ///
    ///	let api = HolidayAPI::new("00000000-0000-0000-0000-000000000000").unwrap();
    /// let request = api.workdays("us", "YYYY-MM-DD", "YYYY-MM-DD");
    /// ```
    pub fn workdays(&self, country: &str, start: &str, days: &str) -> WorkdaysRequest {
        WorkdaysRequest::new(self, country, start, days)
    }

    /// Generates a minimal `languages` request and returns it.
    ///
    /// # Examples
    ///
    /// Basic usage
    /// ```
    /// use holidayapi_rust::HolidayAPI;
    ///
    ///	let api = HolidayAPI::new("00000000-0000-0000-0000-000000000000").unwrap();
    /// let request = api.languages();
    /// ```
    ///
    /// Adding optional parameters with builder pattern
    /// ```
    /// use holidayapi_rust::HolidayAPI;
    ///
    /// let api = HolidayAPI::new("00000000-0000-0000-0000-000000000000").unwrap();
    /// let specific_request = api.languages().search("united states");
    /// ```
    pub fn languages(&self) -> LanguagesRequest {
        LanguagesRequest::new(self)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    static EXPIRED_KEY: &str = "daaaaaab-aaaa-aaaa-aaaa-2aaaada37e14";
    static INVALID_KEY: &str = "invalid-key-format";

    #[test]
    fn test_valid_key() {
        match HolidayAPI::new(EXPIRED_KEY) {
            Ok(_) => assert!(true),
            Err(_) => unreachable!("Should not return an error on valid key"),
        }
        match HolidayAPI::new(INVALID_KEY) {
            Ok(_) => unreachable!("Should return an error on invalid key"),
            Err(_) => assert!(true),
        }
    }

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
