use iso639_enum::Language as ISOLanguage;
use iso_currency::Currency as ISOCurrency;
use isocountry::CountryCode;
use std::{net::IpAddr, str::FromStr};

use super::ListKey;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Locale {
    pub ip: IpAddr,
    pub country_code: CountryCode,
    pub country: String,
    pub continent_code: ContinentCode,
    pub continent: String,
    pub eu: bool,
    pub currency: ISOCurrency,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContinentCode {
    AF,
    AN,
    AS,
    EU,
    NA,
    OC,
    SA,
}

impl FromStr for ContinentCode {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AF" => Ok(ContinentCode::AF),
            "AN" => Ok(ContinentCode::AN),
            "AS" => Ok(ContinentCode::AS),
            "EU" => Ok(ContinentCode::EU),
            "NA" => Ok(ContinentCode::NA),
            "OC" => Ok(ContinentCode::OC),
            "SA" => Ok(ContinentCode::SA),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid continent code",
            )),
        }
    }
}

impl ContinentCode {
    pub fn get_full_name(&self) -> &'static str {
        match self {
            ContinentCode::AF => "Africa",
            ContinentCode::AN => "Antarctica",
            ContinentCode::AS => "Asia",
            ContinentCode::EU => "Europe",
            ContinentCode::NA => "North America",
            ContinentCode::OC => "Oceania",
            ContinentCode::SA => "South America",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Country {
    pub code: CountryCode,
    pub name: String,
}

impl ListKey for Country {
    fn list_key() -> &'static str {
        "countries"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Phone {
    pub code: String,
    pub country_code: CountryCode,
    pub country_name: String,
}

impl ListKey for Phone {
    fn list_key() -> &'static str {
        "phones"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Continent {
    pub code: ContinentCode,
    pub name: String,
}

impl ListKey for Continent {
    fn list_key() -> &'static str {
        "continents"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Currency {
    pub symbol: String,
    pub name: String,
    pub symbol_native: String,
    pub decimal_digits: u8,
    pub rounding: u8,
    pub code: ISOCurrency,
    pub name_plural: String,
}

impl ListKey for Currency {
    fn list_key() -> &'static str {
        "currencies"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Language {
    pub code: ISOLanguage,
    pub name: String,
    pub native_name: String,
}

impl ListKey for Language {
    fn list_key() -> &'static str {
        "languages"
    }
}
