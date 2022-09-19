use std::{
    collections::{hash_map, HashMap},
    error::Error,
};

use regex::Regex;
use reqwest::{Response, Url};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct HolidayAPI {
    base_url: String,
    key: String,
}

#[derive(Debug)]
pub enum HolidayAPIError {
    InvalidKey,
    InvalidVersion(String),
}

#[derive(strum_macros::Display)]
pub enum Endpoint {
    Countries,
    Holidays,
    Languages,
    Workday,
    Workdays,
}

pub enum Request {
    CountriesRequest(CountriesRequest),
}

impl IntoIterator for Request {
    type Item = (String, String);

    type IntoIter = hash_map::IntoIter<String, String>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Request::CountriesRequest(c) => {
                let json = serde_json::to_string(&c).unwrap();
                let map: HashMap<String, String> =
                    serde_json::from_str::<HashMap<Option<String>, Option<String>>>(&json)
                        .unwrap()
                        .into_iter()
                        .filter(|x| x.0.is_some() && x.1.is_some())
                        .map(|(k, v)| (k.unwrap(), v.unwrap()))
                        .collect();
                map.into_iter()
            }
        }
    }
}

#[derive(strum_macros::Display, Debug, Clone, Serialize, Deserialize)]
pub enum Format {
    #[serde(rename = "csv")]
    CSV,
    #[serde(rename = "json")]
    JSON,
    #[serde(rename = "php")]
    PHP,
    #[serde(rename = "tsv")]
    TSV,
    #[serde(rename = "yaml")]
    YAML,
    #[serde(rename = "xml")]
    XML,
}
#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct CountriesRequest {
    country: Option<String>,
    search: Option<String>,
    public: Option<bool>,
    format: Option<Format>,
    pretty: Option<bool>,
}

impl CountriesRequest {
    pub fn new() -> CountriesRequest {
        CountriesRequest {
            country: None,
            search: None,
            public: None,
            format: None,
            pretty: None,
        }
    }
    pub fn country(&mut self, country: &str) -> CountriesRequest {
        self.country = Some(country.to_string());
        self.to_owned()
    }
    pub fn search(&mut self, search: &str) -> CountriesRequest {
        self.search = Some(search.to_string());
        self.clone()
    }
    pub fn public(&mut self, public: bool) -> CountriesRequest {
        self.public = Some(public);
        self.to_owned()
    }

    pub fn format(&mut self, format: Format) -> CountriesRequest {
        self.format = Some(format);
        self.to_owned()
    }

    pub fn pretty(&mut self, pretty: bool) -> CountriesRequest {
        self.pretty = Some(pretty);
        self.to_owned()
    }

    pub async fn send(&self, api: &HolidayAPI) -> Result<Response, Box<dyn Error>> {
        api.request(
            Endpoint::Countries,
            Request::CountriesRequest(self.to_owned()),
        )
        .await
    }
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
        request: Request,
    ) -> Result<Response, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let url = Url::parse(self.base_url.as_str()).unwrap();

        let url = url
            .join(endpoint.to_string().to_ascii_lowercase().as_str())
            .unwrap();

        let url = Url::parse_with_params(&format!("{}?key={}", url, self.key), request).unwrap();

        let response = client.get(url).send().await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXPIRED_KEY: &str = "a112a6bb-a47c-4aa7-b1d2-aaaab24aaacf";

    #[test]
    fn test_working_iterator() {
        let request =
            Request::CountriesRequest(CountriesRequest::new().country("US").format(Format::CSV));
        let iter = request.into_iter();
        println!("{:?}", iter);
        assert!(iter.count() == 2);
    }

    #[tokio::test]
    #[ignore]
    async fn test_request() {
        let api = HolidayAPI::new(EXPIRED_KEY).unwrap();
        let response = CountriesRequest::new()
            .country("US")
            .send(&api)
            .await
            .unwrap();

        assert!(response.status().is_client_error());
    }
}
