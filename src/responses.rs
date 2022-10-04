use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct APIRequests {
    pub available: u32,
    pub used: u32,
    pub resets: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CountriesResponse {
    pub requests: APIRequests,
    pub status: u32,
    pub error: Option<String>,
    pub warning: Option<String>,
    pub countries: Vec<Country>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Country {
    pub code: String,
    pub name: String,
    pub languages: Vec<String>,
    pub codes: Codes,
    pub flag: String,
    pub subdivisions: Vec<Subdivision>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Codes {
    #[serde(rename = "alpha-2")]
    pub alpha_2: String,
    #[serde(rename = "alpha-3")]
    pub alpha_3: String,
    pub numeric: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Subdivision {
    pub code: String,
    pub name: String,
    pub languages: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct HolidaysResponse {
    pub requests: APIRequests,
    pub status: u32,
    pub holidays: Vec<Holiday>,
    pub error: Option<String>,
    pub warning: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Holiday {
    pub name: String,
    pub date: String,
    pub observed: String,
    pub public: bool,
    pub country: String,
    pub uuid: String,
    pub weekday: Weekday,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Weekday {
    pub date: Date,
    pub observed: Date,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Date {
    pub name: String,
    pub numeric: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WorkdayResponse {
    pub requests: APIRequests,
    pub status: u32,
    pub date: String,
    pub weekday: Date,
    pub error: Option<String>,
    pub warning: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]

pub struct WorkdaysResponse {
    pub requests: APIRequests,
    pub status: u32,
    pub workdays: u32,
    pub error: Option<String>,
    pub warning: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]

pub struct LanguagesResponse {
    pub requests: APIRequests,
    pub status: u32,
    pub languages: Vec<Language>,
    pub error: Option<String>,
    pub warning: Option<String>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Language {
    pub code: String,
    pub name: String,
}
