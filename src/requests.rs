use crate::{
    responses::{CountriesResponse, Country, Date, Holiday, HolidaysResponse, WorkdayResponse},
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
    pub fn new(api: &HolidayAPI) -> Self {
        CountriesRequest {
            parameters: HashMap::new(),
            api: api.clone(),
        }
    }

    pub fn country(&mut self, country: &str) -> Self {
        self.parameters.insert("country".into(), country.into());
        self.to_owned()
    }
    pub fn search(&mut self, search: &str) -> Self {
        self.parameters.insert("search".into(), search.into());
        self.to_owned()
    }
    pub fn public(&mut self, public: bool) -> Self {
        self.parameters.insert("public".into(), public.to_string());
        self.to_owned()
    }

    pub fn pretty(&mut self, pretty: bool) -> Self {
        self.parameters.insert("pretty".into(), pretty.to_string());
        self.to_owned()
    }

    pub async fn get_raw(self) -> Result<String, Box<dyn Error>> {
        Ok(self
            .api
            .request(Endpoint::Countries, self.parameters)
            .await?
            .text()
            .await?)
    }

    pub async fn get_full(self) -> Result<CountriesResponse, Box<dyn Error>> {
        Ok(serde_json::from_str(&self.get_raw().await?)?)
    }

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
    pub fn new(api: &HolidayAPI, country: String, year: i32) -> Self {
        let mut holiday = HolidaysRequest {
            parameters: HashMap::new(),
            api: api.clone(),
        };
        holiday.parameters.insert("country".into(), country);
        holiday.parameters.insert("year".into(), year.to_string());
        return holiday;
    }

    pub fn month(&mut self, month: i32) -> Self {
        self.parameters.insert("month".into(), month.to_string());
        self.to_owned()
    }

    pub fn day(&mut self, day: i32) -> Self {
        self.parameters.insert("day".into(), day.to_string());
        self.to_owned()
    }

    pub fn public(&mut self, public: bool) -> Self {
        self.parameters.insert("public".into(), public.to_string());
        self.to_owned()
    }

    pub fn subdivisions(&mut self, subdivisions: bool) -> Self {
        self.parameters
            .insert("subdivisions".into(), subdivisions.to_string());
        self.to_owned()
    }

    pub fn search(&mut self, search: &str) -> Self {
        self.parameters.insert("search".into(), search.to_string());
        self.to_owned()
    }

    pub fn language(&mut self, language: &str) -> Self {
        self.parameters
            .insert("language".into(), language.to_string());
        self.to_owned()
    }

    pub fn previous(&mut self, previous: bool) -> Self {
        self.parameters
            .insert("previous".into(), previous.to_string());
        self.to_owned()
    }

    pub fn upcoming(&mut self, upcoming: bool) -> Self {
        self.parameters
            .insert("upcoming".into(), upcoming.to_string());
        self.to_owned()
    }

    pub fn pretty(&mut self, pretty: bool) -> Self {
        self.parameters.insert("pretty".into(), pretty.to_string());
        self.to_owned()
    }

    pub async fn get_raw(self) -> Result<String, Box<dyn Error>> {
        Ok(self
            .api
            .request(Endpoint::Holidays, self.parameters)
            .await?
            .text()
            .await?)
    }

    pub async fn get_full(self) -> Result<HolidaysResponse, Box<dyn Error>> {
        Ok(serde_json::from_str(&self.get_raw().await?)?)
    }

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
    pub fn new(api: &HolidayAPI, country: &str, start: &str, days: usize) -> WorkdayRequest {
        let mut workday = WorkdayRequest {
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

    pub fn pretty(&mut self, pretty: bool) -> Self {
        self.parameters.insert("pretty".into(), pretty.to_string());
        self.to_owned()
    }

    pub async fn get_raw(self) -> Result<String, Box<dyn Error>> {
        Ok(self
            .api
            .request(Endpoint::Workday, self.parameters)
            .await?
            .text()
            .await?)
    }

    pub async fn get_full(self) -> Result<WorkdayResponse, Box<dyn Error>> {
        Ok(serde_json::from_str(&self.get_raw().await?)?)
    }

    pub async fn get(self) -> Result<(String, Date), Box<dyn Error>> {
        let res = self.get_full().await?;
        Ok((res.date, res.weekday))
    }
}

#[derive(Debug, Clone)]
struct WorkdaysRequest {
    parameters: HashMap<String, String>,
    api: HolidayAPI,
}
