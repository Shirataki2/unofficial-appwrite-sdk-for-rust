use crate::{
    check_response,
    error::Error,
    models::{
        file::{File, InputFile},
        Id,
    },
};

use reqwest::{
    header::{HeaderMap, HeaderValue, IntoHeaderName, InvalidHeaderValue},
    multipart, Method,
};

#[derive(Debug, Clone)]
pub struct AppWriteClientHeader(HeaderMap<HeaderValue>);

impl AppWriteClientHeader {
    pub fn add_header<K: IntoHeaderName, V: Into<HeaderValue>>(&mut self, key: K, value: V) {
        self.0.insert(key, value.into());
    }
}

impl Default for AppWriteClientHeader {
    fn default() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-sdk-version",
            HeaderValue::from_static("appwrite:rust:1.2.0"),
        );
        headers.insert(
            "X-Appwrite-Response-Format",
            HeaderValue::from_static("1.2.0"),
        );
        AppWriteClientHeader(headers)
    }
}

#[derive(Debug)]
pub enum RequestData {
    Json(serde_json::Value),
    FormData(serde_json::Value),
    Params(Vec<(String, String)>),
    None,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AppWriteClient {
    // TODO: Change to private when stable
    pub client: reqwest::Client,
    project_id: String,
    host_url: String,
    self_signed: bool,
    chunk_size: usize,
    headers: AppWriteClientHeader,
}

impl AppWriteClient {
    pub fn builder(host_url: &str, project_id: &str) -> AppWriteClientBuilder {
        AppWriteClientBuilder::new(host_url, project_id)
    }

    pub fn get_host_url(&self) -> &str {
        &self.host_url
    }

    pub fn get_project_id(&self) -> &str {
        &self.project_id
    }

    pub async fn call(
        &self,
        method: Method,
        url: &str,
        content: RequestData,
    ) -> Result<reqwest::Response, Error> {
        let url = format!("{}{}", self.host_url, url);
        let url = match content {
            RequestData::Params(ref params) => {
                let url = url::Url::parse_with_params(&url, params.iter());
                match url {
                    Ok(url) => url.to_string(),
                    Err(err) => return Err(Error::InvalidUrl(err)),
                }
            }
            _ => url,
        };
        let request = self.client.request(method, url);
        let response = match content {
            RequestData::Json(json) => request.json(&json).send().await?,
            RequestData::FormData(json) => request.form(&json).send().await?,
            _ => request.send().await?,
        };
        Ok(response)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn chunk_upload<Resp>(
        &self,
        method: Method,
        url: &str,
        data_param_key: &str,
        id_param_key: &str,
        mut input_file: InputFile,
        data: Vec<(String, String)>,
        additional_headers: Option<HeaderMap<HeaderValue>>,
        upload_id: Option<String>,
    ) -> Result<Option<Resp>, Error>
    where
        Resp: serde::de::DeserializeOwned + Id,
    {
        let url = format!("{}{}", self.host_url, url);
        let filename = input_file.name.clone();
        let mime_type = input_file.mime_type.clone();
        let key = data_param_key.to_string();
        let data = data.clone();

        if input_file.size < self.chunk_size {
            let mut req = self.client.request(method, url.clone());

            if let Some(ref additional_headers) = additional_headers {
                for (key, value) in additional_headers.iter() {
                    req = req.header(key, value);
                }
            }
            let fileform = multipart::Part::stream(input_file)
                .file_name(filename.clone())
                .mime_str(&mime_type.clone())?;
            let mut form = multipart::Form::new();
            for (key, value) in data.iter() {
                form = form.text(key.clone(), value.clone());
            }
            form = form.part(key.clone(), fileform);

            let resp = req.multipart(form).send().await?;
            let resp = check_response!(Resp: resp);
            // let resp = resp.json::<Resp>().await?;
            Ok(Some(resp))
        } else {
            let mut offset = 0;
            let mut counter = 0;
            if let Some(upload_id) = upload_id {
                if upload_id != "unique()" {
                    // Try to resume upload
                    let resp = self
                        .call(
                            Method::GET,
                            &format!("{}/{}", url, upload_id),
                            RequestData::None,
                        )
                        .await?;
                    check_response!(resp);
                    match resp.json::<File>().await {
                        Err(_) => {}
                        Ok(file) => {
                            counter = file.chunks_uploaded as usize;
                        }
                    }
                }
            }
            if counter > 0 {
                offset = counter * self.chunk_size;
                input_file.seek(offset).await?;
            }

            let mut id: Option<String> = None;
            let mut result: Option<Resp> = None;
            loop {
                let method = method.clone();
                let mut req = self.client.request(method, url.clone());

                let mut buf = vec![0; self.chunk_size];
                let n = input_file.read_exact(&mut buf).await?;
                buf.truncate(n);
                if buf.is_empty() {
                    break;
                }
                if let Some(ref additional_headers) = additional_headers {
                    for (key, value) in additional_headers.iter() {
                        req = req.header(key, value);
                    }
                }
                if let Some(ref id) = id {
                    req = req.header("x-appwrite-id", id.clone())
                }
                req = req.header(
                    "content-range",
                    format!(
                        "bytes {}-{}/{}",
                        offset,
                        (offset + self.chunk_size - 1).min(input_file.size),
                        input_file.size
                    ),
                );
                offset += buf.len();

                let fileform = multipart::Part::stream(buf)
                    .file_name(filename.clone())
                    .mime_str(&mime_type.clone())?;
                let mut form = multipart::Form::new();
                for (key, value) in data.iter() {
                    if key == id_param_key {
                        if let Some(ref id) = id {
                            form = form.text(key.clone(), id.clone());
                        } else {
                            form = form.text(key.clone(), value.clone());
                        }
                    } else {
                        form = form.text(key.clone(), value.clone());
                    }
                }
                form = form.part(key.clone(), fileform);

                let resp = req.multipart(form).send().await?;
                check_response!(resp);
                let resp = resp.json::<Resp>().await?;
                id = Some(resp.id().clone());
                result = Some(resp);

                counter += 1;
            }

            Ok(result)
        }
    }
}

pub struct AppWriteClientBuilder {
    host_url: String,
    project_id: String,
    self_signed: bool,
    chunk_size: usize,
    headers: AppWriteClientHeader,
}

impl AppWriteClientBuilder {
    pub fn new(host_url: &str, project_id: &str) -> Self {
        AppWriteClientBuilder {
            host_url: host_url.to_string(),
            project_id: project_id.to_string(),
            self_signed: false,
            chunk_size: 5 * 1024 * 1024,
            headers: AppWriteClientHeader::default(),
        }
    }
    pub fn self_signed(mut self, self_signed: bool) -> Self {
        self.self_signed = self_signed;
        self
    }
    pub fn chunk_size(mut self, chunk_size: usize) -> Self {
        self.chunk_size = chunk_size;
        self
    }
    pub fn add_header<K: IntoHeaderName, V: Into<HeaderValue>>(mut self, key: K, value: V) -> Self {
        self.headers.add_header(key, value);
        self
    }

    pub fn set_jwt_token(mut self, jwt_token: &str) -> Result<Self, InvalidHeaderValue> {
        self.headers
            .add_header("x-appwrite-jwt", HeaderValue::from_str(jwt_token)?);
        Ok(self)
    }

    pub fn set_key(mut self, key: &str) -> Result<Self, InvalidHeaderValue> {
        self.headers
            .add_header("x-appwrite-key", HeaderValue::from_str(key)?);
        Ok(self)
    }

    pub fn set_locale(mut self, locale: &str) -> Result<Self, InvalidHeaderValue> {
        self.headers
            .add_header("x-appwrite-locale", HeaderValue::from_str(locale)?);
        Ok(self)
    }

    pub fn build(self) -> Result<AppWriteClient, Error> {
        let mut headers = self.headers.clone();
        headers.add_header(
            "X-Appwrite-Project",
            HeaderValue::from_str(&self.project_id)?,
        );

        let client = reqwest::Client::builder()
            .gzip(true)
            .default_headers(headers.0)
            .build()
            .map_err(Error::FailedToCreateClient)?;
        Ok(AppWriteClient {
            client,
            project_id: self.project_id,
            host_url: self.host_url,
            self_signed: self.self_signed,
            chunk_size: self.chunk_size,
            headers: self.headers,
        })
    }
}
