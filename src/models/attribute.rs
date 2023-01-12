use super::{collection::CollectionId, database::DatabaseId, DataStatus};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Attribute {
    Boolean(AttributeBoolean),
    Integer(AttributeInteger),
    Double(AttributeDouble),
    String(AttributeStringLike),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "format", rename_all = "camelCase")]
pub enum AttributeStringLike {
    Email(AttributeEmail),
    DateTime(AttributeDateTime),
    Enum(AttributeEnum),
    Url(AttributeUrl),
    Ip(AttributeIp),
    String(AttributeString),
}

impl Attribute {
    pub fn new_boolean(
        key: &str,
        required: bool,
        default: Option<bool>,
        is_array: Option<bool>,
    ) -> Self {
        Attribute::Boolean(AttributeBoolean {
            key: key.to_string(),
            required,
            default_value: default.unwrap_or_default(),
            is_array: is_array.unwrap_or_default(),
            status: DataStatus::default(),
        })
    }

    pub fn new_integer(
        key: &str,
        required: bool,
        default: Option<i64>,
        min: Option<i64>,
        max: Option<i64>,
        is_array: Option<bool>,
    ) -> Self {
        Attribute::Integer(AttributeInteger {
            key: key.to_string(),
            required,
            default_value: default,
            is_array: is_array.unwrap_or_default(),
            min,
            max,
            status: DataStatus::default(),
        })
    }

    pub fn new_double(
        key: &str,
        required: bool,
        default: Option<f64>,
        min: Option<f64>,
        max: Option<f64>,
        is_array: Option<bool>,
    ) -> Self {
        Attribute::Double(AttributeDouble {
            key: key.to_string(),
            required,
            default_value: default,
            is_array: is_array.unwrap_or_default(),
            min,
            max,
            status: DataStatus::default(),
        })
    }

    pub fn new_string(
        key: &str,
        required: bool,
        default: Option<String>,
        size: usize,
        is_array: Option<bool>,
    ) -> Self {
        Attribute::String(AttributeStringLike::String(AttributeString {
            key: key.to_string(),
            required,
            default_value: default.unwrap_or_default(),
            is_array: is_array.unwrap_or_default(),
            size,
            status: DataStatus::default(),
        }))
    }

    pub fn new_email(
        key: &str,
        required: bool,
        default: Option<String>,
        is_array: Option<bool>,
    ) -> Self {
        Attribute::String(AttributeStringLike::Email(AttributeEmail {
            key: key.to_string(),
            required,
            default_value: default.unwrap_or_default(),
            is_array: is_array.unwrap_or_default(),
            status: DataStatus::default(),
        }))
    }

    pub fn new_datetime(
        key: &str,
        required: bool,
        default: Option<String>,
        is_array: Option<bool>,
    ) -> Self {
        Attribute::String(AttributeStringLike::DateTime(AttributeDateTime {
            key: key.to_string(),
            required,
            default_value: default.unwrap_or_default(),
            is_array: is_array.unwrap_or_default(),
            status: DataStatus::default(),
        }))
    }

    pub fn new_url(
        key: &str,
        required: bool,
        default: Option<String>,
        is_array: Option<bool>,
    ) -> Self {
        Attribute::String(AttributeStringLike::Url(AttributeUrl {
            key: key.to_string(),
            required,
            default_value: default.unwrap_or_default(),
            is_array: is_array.unwrap_or_default(),
            status: DataStatus::default(),
        }))
    }

    pub fn new_enum<S>(
        key: &str,
        required: bool,
        default: Option<String>,
        elements: &[S],
        is_array: Option<bool>,
    ) -> Self
    where
        S: AsRef<str> + Clone,
    {
        let elements = elements
            .iter()
            .map(|v| {
                let v: &str = v.as_ref();
                v.to_string()
            })
            .collect::<Vec<_>>();
        Attribute::String(AttributeStringLike::Enum(AttributeEnum {
            key: key.to_string(),
            required,
            default_value: default.unwrap_or_default(),
            is_array: is_array.unwrap_or_default(),
            elements,
            status: DataStatus::default(),
        }))
    }

    pub fn new_ip(
        key: &str,
        required: bool,
        default: Option<String>,
        is_array: Option<bool>,
    ) -> Self {
        Attribute::String(AttributeStringLike::Ip(AttributeIp {
            key: key.to_string(),
            required,
            default_value: default.unwrap_or_default(),
            is_array: is_array.unwrap_or_default(),
            status: DataStatus::default(),
        }))
    }

    pub fn get_url_endpoint(
        &self,
        database_id: &DatabaseId,
        collection_id: &CollectionId,
    ) -> String {
        let base = format!("/databases/{database_id}/collections/{collection_id}/attributes");
        match self {
            Attribute::Boolean(_) => format!("{}/boolean", base),
            Attribute::Integer(_) => format!("{}/integer", base),
            Attribute::Double(_) => format!("{}/float", base),
            Attribute::String(AttributeStringLike::Email(_)) => format!("{}/email", base),
            Attribute::String(AttributeStringLike::DateTime(_)) => format!("{}/datetime", base),
            Attribute::String(AttributeStringLike::Enum(_)) => format!("{}/enum", base),
            Attribute::String(AttributeStringLike::Url(_)) => format!("{}/url", base),
            Attribute::String(AttributeStringLike::Ip(_)) => format!("{}/ip", base),
            Attribute::String(AttributeStringLike::String(_)) => format!("{}/string", base),
        }
    }

    pub fn convert_to_request_body(&self) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::to_value(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AttributeBoolean {
    pub key: String,
    pub status: DataStatus,
    pub required: bool,
    #[serde(rename = "array")]
    pub is_array: bool,
    #[serde(rename = "default")]
    pub default_value: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AttributeInteger {
    pub key: String,
    pub status: DataStatus,
    pub required: bool,
    #[serde(rename = "array")]
    pub is_array: bool,
    #[serde(rename = "default")]
    pub default_value: Option<i64>,
    pub min: Option<i64>,
    pub max: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AttributeDouble {
    pub key: String,
    pub status: DataStatus,
    pub required: bool,
    #[serde(rename = "array")]
    pub is_array: bool,
    #[serde(rename = "default")]
    pub default_value: Option<f64>,
    pub min: Option<f64>,
    pub max: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AttributeEmail {
    pub key: String,
    pub status: DataStatus,
    pub required: bool,
    #[serde(rename = "array")]
    pub is_array: bool,
    #[serde(rename = "default")]
    pub default_value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AttributeDateTime {
    pub key: String,
    pub status: DataStatus,
    pub required: bool,
    #[serde(rename = "array")]
    pub is_array: bool,
    #[serde(rename = "default")]
    pub default_value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AttributeEnum {
    pub key: String,
    pub status: DataStatus,
    pub required: bool,
    #[serde(rename = "array")]
    pub is_array: bool,
    #[serde(rename = "default")]
    pub default_value: String,
    pub elements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AttributeUrl {
    pub key: String,
    pub status: DataStatus,
    pub required: bool,
    #[serde(rename = "array")]
    pub is_array: bool,
    #[serde(rename = "default")]
    pub default_value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AttributeIp {
    pub key: String,
    pub status: DataStatus,
    pub required: bool,
    #[serde(rename = "array")]
    pub is_array: bool,
    #[serde(rename = "default")]
    pub default_value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AttributeString {
    pub key: String,
    pub status: DataStatus,
    pub required: bool,
    #[serde(rename = "array")]
    pub is_array: bool,
    #[serde(rename = "default")]
    pub default_value: String,
    pub size: usize,
}

#[test]
fn test_serialize() {
    let attribute = Attribute::Boolean(AttributeBoolean {
        key: "key".to_string(),
        status: DataStatus::Available,
        required: true,
        is_array: false,
        default_value: true,
    });
    let serialized = serde_json::to_string(&attribute).unwrap();
    println!("{}", serialized);
}

#[test]
fn test_deserialize() {
    let serialized = r#"{"type":"boolean","key":"key","status":"available","required":true,"array":false,"default":true}"#;
    let attribute = serde_json::from_str::<Attribute>(serialized).unwrap();
    println!("{:?}", attribute);
}

#[test]
fn test_deserialize_string() {
    let serialized = r#"{"type":"string","key":"key","status":"available","required":true,"array":false,"default":"test","format":"email"}"#;
    let attribute = serde_json::from_str::<Attribute>(serialized).unwrap();
    println!("{:?}", attribute);
}
