#[macro_export]
macro_rules! check_response {
    ($ty:ty: $response:expr) => {{
        if !$response.status().is_success() {
            return Err(Error::from_response($response).await);
        }
        let data = $response.text().await?;
        println!("{}", data);
        serde_json::from_str::<$ty>(&data)?
    }};
    ($response:expr) => {{
        if !$response.status().is_success() {
            return Err($crate::error::Error::from_response($response).await);
        }
    }};
}

#[macro_export]
macro_rules! insert_if_some {
    ($p:expr => $s:tt, $v: expr) => {
        if let Some(v) = $v {
            $p.push(($s.to_string(), v.to_string()));
        }
    };
    ($p:expr => $s:tt, $v: expr; $($ss:tt, $vs: expr);*) => {
        insert_if_some!($p=> $s, $v);
        insert_if_some!($p=> $($ss, $vs);*);
    };
}
