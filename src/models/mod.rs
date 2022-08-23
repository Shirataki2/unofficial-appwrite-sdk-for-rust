use std::{fmt, marker::PhantomData};

pub mod account;
pub mod attribute;
pub mod avatar;
pub mod bucket;
pub mod collection;
pub mod database;
pub mod deployment;
pub mod document;
pub mod execution;
pub mod file;
pub mod function;
pub mod health;
pub mod index;
pub mod locale;
pub mod log;
pub mod membership;
pub mod permission;
pub mod runtime;
pub mod session;
pub mod team;
pub mod user;

pub type TimeStamp = chrono::DateTime<chrono::Utc>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DataStatus {
    Available,
    Processing,
    Deleting,
    Stuck,
    Failed,
}

impl Default for DataStatus {
    fn default() -> Self {
        DataStatus::Available
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListResponse<T> {
    pub items: Vec<T>,
    pub total: u64,
}

pub trait ListKey {
    fn list_key() -> &'static str;
}

impl<T> serde::Serialize for ListResponse<T>
where
    T: serde::Serialize + ListKey,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry(T::list_key(), &self.items)?;
        map.serialize_entry("total", &self.total)?;
        map.end()
    }
}

enum ListField<T> {
    Items(PhantomData<T>),
    Total,
}

struct ListFieldVisitor<T>(PhantomData<T>);

impl<'de, T> serde::de::Visitor<'de> for ListFieldVisitor<T>
where
    T: ListKey,
{
    type Value = ListField<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!("`{}` or `total`", T::list_key()))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value {
            v if v == T::list_key() => Ok(ListField::Items(PhantomData)),
            "total" => Ok(ListField::Total),
            _ => Err(serde::de::Error::unknown_variant(
                value,
                &["items", "total"],
            )),
        }
    }
}

impl<'de, T> serde::Deserialize<'de> for ListField<T>
where
    T: ListKey,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as serde::Deserializer<'de>>::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_identifier(ListFieldVisitor(PhantomData))
    }
}

struct ListResponseVisitor<T>(PhantomData<T>);

impl<'de, T> serde::de::Visitor<'de> for ListResponseVisitor<T>
where
    T: serde::Deserialize<'de> + ListKey,
{
    type Value = ListResponse<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("map")
    }
    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: serde::de::MapAccess<'de>,
    {
        let mut items = Vec::new();
        let mut total = None;
        while let Some(key) = map.next_key::<ListField<T>>()? {
            match key {
                ListField::Items(_) => {
                    let v: Vec<T> = map.next_value()?;
                    items.extend(v);
                }
                ListField::Total => {
                    total = Some(map.next_value()?);
                }
            }
        }
        Ok(ListResponse {
            items,
            total: total.unwrap_or(0),
        })
    }
}

impl<'de, T> serde::Deserialize<'de> for ListResponse<T>
where
    T: serde::Deserialize<'de> + ListKey,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as serde::Deserializer<'de>>::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(ListResponseVisitor(PhantomData))
    }
}

#[test]
fn test_serialize() {
    #[derive(Serialize, Deserialize)]
    struct S {
        pub name: String,
    }
    impl ListKey for S {
        fn list_key() -> &'static str {
            "s"
        }
    }
    let list = ListResponse {
        items: vec![S {
            name: "piyo".to_string(),
        }],
        total: 1,
    };
    let serialized = serde_json::to_string(&list).unwrap();
    let expected = r#"{"s":[{"name":"piyo"}],"total":1}"#;
    assert_eq!(expected, serialized);
}

#[test]
fn test_deserialize() {
    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct S {
        pub name: String,
    }
    impl ListKey for S {
        fn list_key() -> &'static str {
            "s"
        }
    }
    let serialized = r#"{"s":[{"name":"piyo"}],"total":1}"#;
    let list = serde_json::from_str::<ListResponse<S>>(serialized).unwrap();
    let expected = ListResponse {
        items: vec![S {
            name: "piyo".to_string(),
        }],
        total: 1,
    };
    assert_eq!(expected, list);
}

pub trait HasId {
    fn id(&self) -> String;
}
