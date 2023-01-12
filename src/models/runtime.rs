use smart_default::SmartDefault;
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

use super::ListKey;

#[derive(Debug, Clone, Serialize_enum_str, Deserialize_enum_str, SmartDefault)]
pub enum ExecutionRuntime {
    #[serde(rename = "php-8.0")]
    Php80,
    #[serde(rename = "php-8.1")]
    Php81,
    #[serde(rename = "node-14.6")]
    Node146,
    #[default]
    #[serde(rename = "node-16.0")]
    Node160,
    #[serde(rename = "node-18.0")]
    Node180,
    #[serde(rename = "ruby-3.0")]
    Ruby30,
    #[serde(rename = "ruby-3.1")]
    Ruby31,
    #[serde(rename = "python-3.8")]
    Python38,
    #[serde(rename = "python-3.9")]
    Python39,
    #[serde(rename = "python-3.10")]
    Python310,
    #[serde(rename = "deno-1.21")]
    Deno121,
    #[serde(rename = "deno-1.24")]
    Deno124,
    #[serde(rename = "dart-2.15")]
    Dart215,
    #[serde(rename = "dart-2.16")]
    Dart216,
    #[serde(rename = "dart-2.17")]
    Dart217,
    #[serde(rename = "dotnet-3.1")]
    Dotnet31,
    #[serde(rename = "dotnet-6.0")]
    Dotnet60,
    #[serde(rename = "java-8.0")]
    Java80,
    #[serde(rename = "java-11.0")]
    Java110,
    #[serde(rename = "java-17.0")]
    Java170,
    #[serde(rename = "java-18.0")]
    Java180,
    #[serde(rename = "swift-5.5")]
    Swift55,
    #[serde(rename = "kotlin-1.6")]
    Kotlin16,
    #[serde(rename = "cpp-17.0")]
    Cpp170,

    #[serde(other)]
    Other(String),
}

impl ListKey for ExecutionRuntime {
    fn list_key() -> &'static str {
        "runtimes"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Runtime {
    #[serde(rename = "$id")]
    pub id: ExecutionRuntime,
    pub name: String,
    pub version: String,
    pub base: String,
    pub image: String,
    pub logo: String,
    pub supports: Vec<String>,
}

impl ListKey for Runtime {
    fn list_key() -> &'static str {
        "runtimes"
    }
}
