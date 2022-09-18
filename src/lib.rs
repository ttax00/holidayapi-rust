pub struct HolidayAPI {
    base_url: String,
    key: String,
}


impl HolidayAPI {
    pub fn new(key: &str, version: Option<u32>) -> HolidayAPI {
        let url = {
            // defaults to version 1
            let mut ver = 1;
            if let Some(v) = version {
                ver = v
            }
            format!("https://holidayapi.com/v{}/", ver)
        };

        HolidayAPI {
            base_url: url,
            key: key.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn test_create_api() {
		let new = HolidayAPI::new("12345abc-de67-89fa-0hba-aadb32avd87", None);
		let truth = HolidayAPI{
			base_url: "https://holidayapi.com/v1/".to_owned(),
			key: "12345abc-de67-89fa-0hba-aadb32avd87".to_owned(),
		};
		assert!(new.base_url == truth.base_url);
		assert!(new.key == truth.key);
	}
}
