# Holiday API Rust Library

[![License](https://img.shields.io/github/license/TechTheAwesome/holidayapi-rust?style=for-the-badge)](https://github.com/TechTheAwesome/holidayapi-rust/blob/main/LICENSE)
[![Test Status](https://img.shields.io/github/workflow/status/techtheawesome/holidayapi-rust/Rust?style=for-the-badge)](https://github.com/holidayapi/holidayapi-node/actions)
![Crates.io](https://img.shields.io/crates/v/holidayapi_rust?style=for-the-badge)
[![wakatime](https://wakatime.com/badge/user/4312729e-bc28-4bc0-9074-161a64a7ad20/project/5efdb06a-8b53-4d90-83bb-1f47a23a30c6.svg?style=for-the-badge)](https://wakatime.com/badge/user/4312729e-bc28-4bc0-9074-161a64a7ad20/project/5efdb06a-8b53-4d90-83bb-1f47a23a30c6)

Unofficial library for [Holiday API](https://holidayapi.com) written in Rust. This repo implements interface for original HolidayAPI endpoints seen [here](https://holidayapi.com/docs).

## Acknowledgments

This project is heavily inspired by [holidayapi-node](https://github.com/holidayapi/holidayapi-node) and [holiday-api-rust](https://github.com/guibranco/holiday-api-rust) repositories. 

## Installation
Add the following to your `Cargo.toml`

```toml
[dependencies]
holidayapi_rust = "0.1.7"
```

Or use `cargo` cli:
```console
$ cargo add holidayapi_rust
```
## Usage
### Basic
```rust
use holidayapi_rust::prelude::*;

let holiday_api = HolidayAPI::new("00000000-0000-0000-0000-000000000000").unwrap();

async fn main() {
	// Fetch supported countries and subdivisions
	let countries: Vec<Country> = holiday_api.countries().get().await.unwrap();

	// Fetch supported languages
	let languages: Vec<Language> = holiday_api.languages().get().await.unwrap();

	// Fetch holidays with minimum parameters
	let holidays: Vec<Holiday> = holiday_api.holidays("US", 2020).get().await.unwrap();
}
```
### Builder pattern
```rust
use holidayapi_rust::prelude::*;

let holiday_api = HolidayAPI::new(VALID_KEY).unwrap();

// Holidays
let specific_request: Vec<Holiday> = holiday_api
	.holidays("jp", 2021)
	.pretty(true)
	.language("cn")
	.public(true)
	.get()
	.await; 

// Countries
let specific_request: Vec<Country> = holiday_api
	.countries()
	.search("hello world")
	.country("US")
	.public(true)
	.get()
	.await
	.unwrap();

// Languages
let specific_request: Vec<Language> = holiday_api
	.languages()
	.search("chinese")
	.pretty(true)
	.get()
	.await
	.unwrap();
```
## Future ideas
- [ ] Refactor async call using [IntoFuture](https://doc.rust-lang.org/std/future/trait.IntoFuture.html) to remove unnecessary `.get()` calls.
- [ ] Implements memoization for api calls.
- [ ] Add new utility functions on top of raw API. 