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
    pub fn country(&mut self, country: &str) -> &mut CountriesRequest {
        self.country = Some(country.to_string());
        self
    }
    pub fn search(&mut self, search: &str) -> &mut CountriesRequest {
        self.search = Some(search.to_string());
        self
    }
    pub fn public(&mut self, public: bool) -> &mut CountriesRequest {
        self.public = Some(public);
        self
    }

    pub fn format(&mut self, format: Format) -> &mut CountriesRequest {
        self.format = Some(format);
        self
    }

    pub fn pretty(&mut self, pretty: bool) -> &mut CountriesRequest {
        self.pretty = Some(pretty);
        self
    }

    pub fn build(&self) -> CountriesRequest {
        self.to_owned()
    }
}

impl HolidayAPI {
    fn is_valid_key(key: &str) -> bool {
        let uuid_regex =
            Regex::new(r"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}")
                .expect("Regex is correct");

        // uuid_regex.is_match(key)
        true
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

    pub async fn countries(&self, request: CountriesRequest) -> Result<Response, Box<dyn Error>> {
        self.request(Endpoint::Countries, Request::CountriesRequest(request))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_working_iterator() {
        let request = Request::CountriesRequest(
            CountriesRequest::new()
                .country("US")
                .format(Format::CSV)
                .build(),
        );
        let mut iter = request.into_iter();
        println!("{:?}", iter);
        assert!(iter.next() == Some(("country".to_owned(), "US".to_owned())));
        assert!(iter.next() == Some(("format".to_owned(), "csv".to_owned())));
        assert!(iter.count() == 0);
    }

    #[tokio::test]
    async fn test_request() {
        let test = HolidayAPI::new("invalid_key").unwrap();
        let response = test
            .countries(CountriesRequest::new().country("US").build())
            .await
            .unwrap();
        assert!(response.status().is_client_error());
    }
}
