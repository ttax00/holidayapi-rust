use regex::Regex;

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
}

#[cfg(test)]
mod tests {}
