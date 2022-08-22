#[macro_export]
macro_rules! check_response {
    ($ty:ty: $response:expr) => {{
        if !$response.status().is_success() {
            return Err(Error::from_response($response).await);
        }
        let data = $response.text().await?;
        serde_json::from_str::<$ty>(&data)?
    }};
    ($response:expr) => {{
        if !$response.status().is_success() {
            return Err(Error::from_response($response).await);
        }
    }};
}
