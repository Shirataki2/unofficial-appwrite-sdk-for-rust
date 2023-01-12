use crate::models::ErrorResponse;
use reqwest::header::InvalidHeaderValue;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to send")]
    SendFailed,
    #[error("{0}")]
    IOError(#[from] std::io::Error),
    #[error("Error while creating client: {0}")]
    FailedToCreateClient(#[source] reqwest::Error),
    #[error("Error while sending request: {0}")]
    Request(#[source] reqwest::Error),
    #[error("Error while parsing response: {0}")]
    ParseResponse(#[source] serde_json::Error),
    #[error("Invalid Header Value: {0}")]
    InvalidHeaderValue(#[from] InvalidHeaderValue),
    #[error("Invalid Response Body: {0}")]
    InvalidResponseBody(reqwest::Error),
    #[error("Invalid Url: {0}")]
    InvalidUrl(url::ParseError),
    #[error("{{0.message}}")]
    ApiError(ErrorResponse),
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        error!("{:?}", error);
        Error::Request(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        error!("Deserialize Error: {:?}", error);
        Error::ParseResponse(error)
    }
}

impl Error {
    pub async fn from_response(response: reqwest::Response) -> Self {
        let body = response.text().await;
        let body = match body {
            Ok(body) => body,
            Err(error) => {
                error!("Error while parsing response: {:?}", error);
                return Error::InvalidResponseBody(error);
            }
        };
        let error = match serde_json::from_str::<ErrorResponse>(&body) {
            Ok(error) => error,
            Err(error) => {
                error!("Error while parsing response: {:?}", error);
                return Error::ParseResponse(error);
            }
        };
        Error::ApiError(error)
    }
}
