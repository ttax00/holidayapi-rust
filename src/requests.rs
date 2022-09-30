use crate::{
    responses::{
        CountriesResponse, Country, Date, Holiday, HolidaysResponse, Language, LanguagesResponse,
        WorkdayResponse, WorkdaysResponse,
    },
    HolidayAPI,
};
use std::{collections::HashMap, error::Error};

#[derive(strum_macros::Display)]
pub enum Endpoint {
    Countries,
    Holidays,
    Languages,
    Workday,
    Workdays,
}

#[derive(Debug, Clone)]
pub struct CountriesRequest {
    parameters: HashMap<String, String>,
    api: HolidayAPI,
}

impl CountriesRequest {
    pub(crate) fn new(api: &HolidayAPI) -> Self {
        Self {
            parameters: HashMap::new(),
            api: api.clone(),
        }
    }

    /// Return only the country with the specified code.
    ///
    /// # Examples
    /// ```
    /// use holidayapi_rust::HolidayAPI;
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
    /// use holidayapi_rust::HolidayAPI;
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
    /// use holidayapi_rust::HolidayAPI;
    ///
    /// let api = HolidayAPI::new("00000000-0000-0000-0000-000000000000").unwrap();
    /// let request = api.countries().public(true);
    /// ```
    pub fn public(&mut self, public: bool) -> Self {
        self.parameters.insert("public".into(), public.to_string());
        self.to_owned()
    }

    /// Response format (csv, json, php, tsv, yaml and xml). Defaults to JSON.
    ///
    /// # Examples
    /// ```
    /// use holidayapi_rust::HolidayAPI;
    ///
    /// let api = HolidayAPI::new("00000000-0000-0000-0000-000000000000").unwrap();
    /// let request = api.countries().pretty(true);
    /// ```
    pub fn pretty(&mut self, pretty: bool) -> Self {
        self.parameters.insert("pretty".into(), pretty.to_string());
        self.to_owned()
    }

    /// Return the raw string response if the request was successful.
    pub async fn get_raw(self) -> Result<String, Box<dyn Error>> {
        Ok(self
            .api
            .request(Endpoint::Countries, self.parameters)
            .await?
            .text()
            .await?)
    }

    /// Parse the raw response and returns the full `CountriesResponse` struct.
    pub async fn get_full(self) -> Result<CountriesResponse, Box<dyn Error>> {
        Ok(serde_json::from_str(&self.get_raw().await?)?)
    }

    /// Returns the important `Vec<Country>` field.
    pub async fn get(self) -> Result<Vec<Country>, Box<dyn Error>> {
        Ok(self.get_full().await?.countries)
    }
}

#[derive(Debug, Clone)]
pub struct HolidaysRequest {
    parameters: HashMap<String, String>,
    api: HolidayAPI,
}

impl HolidaysRequest {
    pub(crate) fn new(api: &HolidayAPI, country: String, year: i32) -> Self {
        let mut holiday = Self {
            parameters: HashMap::new(),
            api: api.clone(),
        };
        holiday.parameters.insert("country".into(), country);
        holiday.parameters.insert("year".into(), year.to_string());
        return holiday;
    }

    /// 1 or 2 digit month (1-12).
    ///
    /// # Examples
    /// ```
    /// use holidayapi_rust::HolidayAPI;
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
    /// use holidayapi_rust::HolidayAPI;
    /// let api = HolidayAPI::new("00000000-0000-0000-0000-000000000000").unwrap();
    ///
    /// let request = api.holidays("JP", 2020).month(12).day(20);
    /// ```
    pub fn day(&mut self, day: i32) -> Self {
        self.parameters.insert("day".into(), day.to_string());
        self.to_owned()
    }

    /// Return only public holidays.
    pub fn public(&mut self, public: bool) -> Self {
        self.parameters.insert("public".into(), public.to_string());
        self.to_owned()
    }

    /// Return state / province holidays alongside countrywide holidays.
    pub fn subdivisions(&mut self, subdivisions: bool) -> Self {
        self.parameters
            .insert("subdivisions".into(), subdivisions.to_string());
        self.to_owned()
    }

    /// Search holidays by name. Minimum 5 characters.
    /// # Examples
    /// ```
    /// use holidayapi_rust::HolidayAPI;
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
    /// use holidayapi_rust::HolidayAPI;
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
    pub fn previous(&mut self, previous: bool) -> Self {
        self.parameters
            .insert("previous".into(), previous.to_string());
        self.to_owned()
    }

    /// Return the first day of holidays that occur after the specific date. month and day are required.
    ///
    /// Cannot be used with previous.
    pub fn upcoming(&mut self, upcoming: bool) -> Self {
        self.parameters
            .insert("upcoming".into(), upcoming.to_string());
        self.to_owned()
    }

    /// Prettifies results to be more human-readable.
    pub fn pretty(&mut self, pretty: bool) -> Self {
        self.parameters.insert("pretty".into(), pretty.to_string());
        self.to_owned()
    }

    /// Return the raw string response if the request was successful.
    pub async fn get_raw(self) -> Result<String, Box<dyn Error>> {
        Ok(self
            .api
            .request(Endpoint::Holidays, self.parameters)
            .await?
            .text()
            .await?)
    }

    /// Parse the raw response and returns the full `HolidaysResponse` struct.
    pub async fn get_full(self) -> Result<HolidaysResponse, Box<dyn Error>> {
        Ok(serde_json::from_str(&self.get_raw().await?)?)
    }

    /// Returns only the important `Vec<Holiday>` field.
    pub async fn get(self) -> Result<Vec<Holiday>, Box<dyn Error>> {
        Ok(self.get_full().await?.holidays)
    }
}

#[derive(Debug, Clone)]
pub struct WorkdayRequest {
    parameters: HashMap<String, String>,
    api: HolidayAPI,
}

impl WorkdayRequest {
    pub(crate) fn new(api: &HolidayAPI, country: &str, start: &str, days: i32) -> Self {
        let mut workday = Self {
            parameters: HashMap::new(),
            api: api.clone(),
        };
        workday
            .parameters
            .insert("country".into(), country.to_string());
        workday.parameters.insert("year".into(), start.to_string());
        workday.parameters.insert("days".into(), days.to_string());
        return workday;
    }

    // Prettifies results to be more human-readable.
    pub fn pretty(&mut self, pretty: bool) -> Self {
        self.parameters.insert("pretty".into(), pretty.to_string());
        self.to_owned()
    }

    /// Return the raw string response if the request was successful.
    pub async fn get_raw(self) -> Result<String, Box<dyn Error>> {
        Ok(self
            .api
            .request(Endpoint::Workday, self.parameters)
            .await?
            .text()
            .await?)
    }

    /// Parse the raw response and returns the full `WorkdayResponse` struct.
    pub async fn get_full(self) -> Result<WorkdayResponse, Box<dyn Error>> {
        Ok(serde_json::from_str(&self.get_raw().await?)?)
    }

    /// Returns only the important `("YYYY-MM-DD", Weekday)` tuple.
    pub async fn get(self) -> Result<(String, Date), Box<dyn Error>> {
        let res = self.get_full().await?;
        Ok((res.date, res.weekday))
    }
}

#[derive(Debug, Clone)]
pub struct WorkdaysRequest {
    parameters: HashMap<String, String>,
    api: HolidayAPI,
}

impl WorkdaysRequest {
    pub fn new(api: &HolidayAPI, country: &str, start: &str, days: &str) -> Self {
        let mut workdays = Self {
            parameters: HashMap::new(),
            api: api.clone(),
        };
        workdays
            .parameters
            .insert("country".into(), country.to_string());
        workdays.parameters.insert("year".into(), start.to_string());
        workdays.parameters.insert("end".into(), days.to_string());
        return workdays;
    }

    /// Prettifies results to be more human-readable.
    pub fn pretty(&mut self, pretty: bool) -> Self {
        self.parameters.insert("pretty".into(), pretty.to_string());
        self.to_owned()
    }

    /// Return the raw string response if the request was successful.
    pub async fn get_raw(self) -> Result<String, Box<dyn Error>> {
        Ok(self
            .api
            .request(Endpoint::Workdays, self.parameters)
            .await?
            .text()
            .await?)
    }

    /// Parse the raw response and returns the full `WorkdaysResponse` struct.
    pub async fn get_full(self) -> Result<WorkdaysResponse, Box<dyn Error>> {
        Ok(serde_json::from_str(&self.get_raw().await?)?)
    }

    /// Returns the number of working / business days between the specified start and end dates.
    pub async fn get(self) -> Result<u32, Box<dyn Error>> {
        let res = self.get_full().await?;
        Ok(res.workdays)
    }
}

#[derive(Debug, Clone)]
pub struct LanguagesRequest {
    parameters: HashMap<String, String>,
    api: HolidayAPI,
}

impl LanguagesRequest {
    pub fn new(api: &HolidayAPI) -> Self {
        Self {
            parameters: HashMap::new(),
            api: api.clone(),
        }
    }

    /// Return only the language with the specified code.
    /// # Examples
    /// ```
    /// use holidayapi_rust::HolidayAPI;
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
    /// use holidayapi_rust::HolidayAPI;
    ///
    /// let api = HolidayAPI::new("00000000-0000-0000-0000-000000000000").unwrap();
    /// let request = api.languages().search("Japan");
    /// ```
    pub fn search(&mut self, search: &str) -> Self {
        self.parameters.insert("search".into(), search.into());
        self.to_owned()
    }

    /// Prettifies results to be more human-readable.
    pub fn pretty(&mut self, pretty: bool) -> Self {
        self.parameters.insert("pretty".into(), pretty.to_string());
        self.to_owned()
    }

    /// Return the raw string response if the request was successful.
    pub async fn get_raw(self) -> Result<String, Box<dyn Error>> {
        Ok(self
            .api
            .request(Endpoint::Languages, self.parameters)
            .await?
            .text()
            .await?)
    }

    /// Parse the raw response and returns the full `LanguagesResponse` struct.
    pub async fn get_full(self) -> Result<LanguagesResponse, Box<dyn Error>> {
        Ok(serde_json::from_str(&self.get_raw().await?)?)
    }

    /// Returns `Vec<Language>` based on your request parameters.
    pub async fn get(self) -> Result<Vec<Language>, Box<dyn Error>> {
        let res = self.get_full().await?;
        Ok(res.languages)
    }
}
