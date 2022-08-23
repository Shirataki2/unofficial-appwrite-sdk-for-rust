use reqwest::Method;

use crate::{
    check_response,
    client::{AppWriteClient, RequestData},
    error::Error,
    models::{
        locale::{Continent, Country, Currency, Language, Locale, Phone},
        ListResponse,
    },
};

pub struct LocalesService;

impl LocalesService {
    pub async fn get_user_locale(client: &AppWriteClient) -> Result<Locale, crate::error::Error> {
        let url = "/locale";
        let response = client.call(Method::GET, url, RequestData::None).await?;
        Ok(check_response!(Locale: response))
    }
    pub async fn list_countries(client: &AppWriteClient) -> Result<ListResponse<Country>, Error> {
        let url = "/locale/countries";
        let response = client.call(Method::GET, url, RequestData::None).await?;
        Ok(check_response!(ListResponse<Country>: response))
    }
    pub async fn list_eu_countries(
        client: &AppWriteClient,
    ) -> Result<ListResponse<Country>, Error> {
        let url = "/locale/countries/eu";
        let response = client.call(Method::GET, url, RequestData::None).await?;
        Ok(check_response!(ListResponse<Country>: response))
    }
    pub async fn list_currencies(client: &AppWriteClient) -> Result<ListResponse<Currency>, Error> {
        let url = "/locale/currencies";
        let response = client.call(Method::GET, url, RequestData::None).await?;
        Ok(check_response!(ListResponse<Currency>: response))
    }
    pub async fn list_languages(client: &AppWriteClient) -> Result<ListResponse<Language>, Error> {
        let url = "/locale/languages";
        let response = client.call(Method::GET, url, RequestData::None).await?;
        Ok(check_response!(ListResponse<Language>: response))
    }
    pub async fn list_phone_codes(client: &AppWriteClient) -> Result<ListResponse<Phone>, Error> {
        let url = "/locale/countries/phones";
        let response = client.call(Method::GET, url, RequestData::None).await?;
        Ok(check_response!(ListResponse<Phone>: response))
    }
    pub async fn list_continents(
        client: &AppWriteClient,
    ) -> Result<ListResponse<Continent>, Error> {
        let url = "/locale/continents";
        let response = client.call(Method::GET, url, RequestData::None).await?;
        Ok(check_response!(ListResponse<Continent>: response))
    }
}
