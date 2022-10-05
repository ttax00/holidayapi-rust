use serde::de::DeserializeOwned;

use crate::{
    responses::{
        CountriesResponse, Country, Date, Holiday, HolidaysResponse, Language, LanguagesResponse,
        WorkdayResponse, WorkdaysResponse,
    },
    Endpoint, HolidayAPI,
};
use std::{collections::HashMap, error::Error, marker::PhantomData};

#[derive(Debug, Clone)]
pub struct Request<T: Clone> {
    parameters: HashMap<String, String>,
    api: HolidayAPI,
    _marker: PhantomData<T>,
}

impl<T> Request<T>
where
    T: Clone + DeserializeOwned,
{
    /// Response format (csv, json, php, tsv, yaml and xml). Defaults to JSON.
    /// Only work with `request.get_raw()`
    pub fn format(&mut self, format: &str) -> Self {
        self.parameters.insert("format".into(), format.into());
        self.to_owned()
    }

    /// Prettifies results to be more human-readable.
    pub fn pretty(&mut self) -> Self {
        self.parameters.insert("pretty".into(), "true".into());
        self.to_owned()
    }

    /// Return the raw String of the response
    pub async fn get_raw(self) -> Result<String, Box<dyn Error>> {
        Ok(self
            .api
            .custom_request(Endpoint::Countries, self.parameters)
            .await?
            .text()
            .await?)
    }

    /// Returns the parsed struct of the response if successful
    pub async fn get_full(self) -> Result<T, Box<dyn Error>> {
        let mut param = self.parameters;
        param.insert("format".into(), "json".into());
        let response = self.api.custom_request(Endpoint::Countries, param).await?;
        Ok(serde_json::from_str(response.text().await?.as_str())?)
    }
}

impl Request<CountriesResponse> {
    pub(crate) fn new(api: &HolidayAPI) -> Self {
        Self {
            parameters: HashMap::new(),
            api: api.clone(),
            _marker: PhantomData,
        }
    }

    /// Return only the country with the specified code.
    ///
    /// # Examples
    /// ```
    /// use holidayapi_rust::prelude::*;
    ///
    /// let api = HolidayAPI::new("00000000-0000-0000-0000-000000000000").unwrap();
    /// let request = api.countries().country("us");
    /// ```
    pub fn country(&mut self, country: &str) -> Self {
        self.parameters.insert("country".into(), country.into());
        self.to_owned()
    }

    /// Search countries by code and name. Minimum 2 characters.
    ///
    /// # Examples
    /// ```
    /// use holidayapi_rust::prelude::*;
    ///
    /// let api = HolidayAPI::new("00000000-0000-0000-0000-000000000000").unwrap();
    /// let request = api.countries().search("Japan");
    /// ```
    pub fn search(&mut self, search: &str) -> Self {
        self.parameters.insert("search".into(), search.into());
        self.to_owned()
    }

    /// Return only countries that have public holidays.
    ///
    /// # Examples
    /// ```
    /// use holidayapi_rust::prelude::*;
    ///
    /// let api = HolidayAPI::new("00000000-0000-0000-0000-000000000000").unwrap();
    /// let request = api.countries().public();
    /// ```
    pub fn public(&mut self) -> Self {
        self.parameters.insert("public".into(), "true".to_string());
        self.to_owned()
    }

    /// Returns only the important `Vec<Holiday>` field.
    pub async fn get(self) -> Result<Vec<Country>, Box<dyn Error>> {
        Ok(self.get_full().await?.countries)
    }
}

impl Request<HolidaysResponse> {
    pub(crate) fn new(api: &HolidayAPI, country: String, year: i32) -> Self {
        let mut holiday = Self {
            parameters: HashMap::new(),
            api: api.clone(),
            _marker: PhantomData,
        };
        holiday.parameters.insert("country".into(), country);
        holiday.parameters.insert("year".into(), year.to_string());
        return holiday;
    }

    /// 1 or 2 digit month (1-12).
    ///
    /// # Examples
    /// ```
    /// use holidayapi_rust::prelude::*;
    /// let api = HolidayAPI::new("00000000-0000-0000-0000-000000000000").unwrap();
    ///
    /// let request = api.holidays("JP", 2020).month(12);
    /// ```
    pub fn month(&mut self, month: i32) -> Self {
        self.parameters.insert("month".into(), month.to_string());
        self.to_owned()
    }

    /// 1 or 2 digit day (1-31 depending on the month). Must be used with `month`.
    ///
    /// # Examples
    /// ```
    /// use holidayapi_rust::prelude::*;
    /// let api = HolidayAPI::new("00000000-0000-0000-0000-000000000000").unwrap();
    ///
    /// let request = api.holidays("JP", 2020).month(12).day(20);
    /// ```
    pub fn day(&mut self, day: i32) -> Self {
        self.parameters.insert("day".into(), day.to_string());
        self.to_owned()
    }

    /// Return only public holidays.
    pub fn public(&mut self) -> Self {
        self.parameters.insert("public".into(), "true".into());
        self.to_owned()
    }

    /// Return state / province holidays alongside countrywide holidays.
    pub fn subdivisions(&mut self) -> Self {
        self.parameters.insert("subdivisions".into(), "true".into());
        self.to_owned()
    }

    /// Search holidays by name. Minimum 5 characters.
    /// # Examples
    /// ```
    /// use holidayapi_rust::prelude::*;
    /// let api = HolidayAPI::new("00000000-0000-0000-0000-000000000000").unwrap();
    ///
    /// let request = api.holidays("JP", 2020).search("independence day");
    /// ```
    pub fn search(&mut self, search: &str) -> Self {
        self.parameters.insert("search".into(), search.to_string());
        self.to_owned()
    }

    /// ISO 639-1 format (with exceptions).
    /// Click [here](https://holidayapi.com/languages) for supported languages.
    ///
    /// # Examples
    /// ```
    /// use holidayapi_rust::prelude::*;
    /// let api = HolidayAPI::new("00000000-0000-0000-0000-000000000000").unwrap();
    ///
    /// let request = api.holidays("JP", 2020).language("en");
    /// ```
    pub fn language(&mut self, language: &str) -> Self {
        self.parameters
            .insert("language".into(), language.to_string());
        self.to_owned()
    }

    /// Return the first day of holidays that occur before the specific date. month and day are required.
    ///
    /// Cannot be used with `upcoming`.
    pub fn previous(&mut self) -> Self {
        self.parameters.insert("previous".into(), "true".into());
        self.to_owned()
    }

    /// Return the first day of holidays that occur after the specific date. month and day are required.
    ///
    /// Cannot be used with previous.
    pub fn upcoming(&mut self) -> Self {
        self.parameters.insert("upcoming".into(), "true".into());
        self.to_owned()
    }

    /// Returns only the important `Vec<Holiday>` field.
    pub async fn get(self) -> Result<Vec<Holiday>, Box<dyn Error>> {
        Ok(self.get_full().await?.holidays)
    }
}

impl Request<WorkdayResponse> {
    pub(crate) fn new(api: &HolidayAPI, country: &str, start: &str, days: i32) -> Self {
        let mut workday = Self {
            parameters: HashMap::new(),
            api: api.clone(),
            _marker: PhantomData,
        };
        workday
            .parameters
            .insert("country".into(), country.to_string());
        workday.parameters.insert("start".into(), start.to_string());
        workday.parameters.insert("days".into(), days.to_string());
        return workday;
    }

    /// Returns only the important `("YYYY-MM-DD", Weekday)` tuple.
    pub async fn get(self) -> Result<(String, Date), Box<dyn Error>> {
        let res = self.get_full().await?;
        Ok((res.date, res.weekday))
    }
}

impl Request<WorkdaysResponse> {
    pub fn new(api: &HolidayAPI, country: &str, start: &str, days: &str) -> Self {
        let mut workdays = Self {
            parameters: HashMap::new(),
            api: api.clone(),
            _marker: PhantomData,
        };
        workdays
            .parameters
            .insert("country".into(), country.to_string());
        workdays
            .parameters
            .insert("start".into(), start.to_string());
        workdays.parameters.insert("end".into(), days.to_string());
        return workdays;
    }

    /// Returns the number of working / business days between the specified start and end dates.
    pub async fn get(self) -> Result<u32, Box<dyn Error>> {
        let res = self.get_full().await?;
        Ok(res.workdays)
    }
}

impl Request<LanguagesResponse> {
    pub fn new(api: &HolidayAPI) -> Self {
        Self {
            parameters: HashMap::new(),
            api: api.clone(),
            _marker: PhantomData,
        }
    }

    /// Return only the language with the specified code.
    /// # Examples
    /// ```
    /// use holidayapi_rust::prelude::*;
    ///
    /// let api = HolidayAPI::new("00000000-0000-0000-0000-000000000000").unwrap();
    /// let request = api.languages().language("us");
    /// ```
    pub fn language(&mut self, language: &str) -> Self {
        self.parameters.insert("language".into(), language.into());
        self.to_owned()
    }

    /// Search languages by code and name. Minimum 2 characters.
    /// # Examples
    /// ```
    /// use holidayapi_rust::prelude::*;
    ///
    /// let api = HolidayAPI::new("00000000-0000-0000-0000-000000000000").unwrap();
    /// let request = api.languages().search("Japan");
    /// ```
    pub fn search(&mut self, search: &str) -> Self {
        self.parameters.insert("search".into(), search.into());
        self.to_owned()
    }

    /// Returns `Vec<Language>` based on your request parameters.
    pub async fn get(self) -> Result<Vec<Language>, Box<dyn Error>> {
        let res = self.get_full().await?;
        Ok(res.languages)
    }
}
