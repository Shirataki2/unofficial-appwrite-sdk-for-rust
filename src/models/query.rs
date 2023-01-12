use std::fmt::{Display, self};

use crate::prelude::DocumentId;

#[derive(Debug, Clone)]
pub struct QueryAttr(String);

impl QueryAttr {
    pub fn new<S>(attr: S) -> Self
    where
        S: Into<String>,
    {
        QueryAttr(format!("\"{}\"", attr.into()))
    }
}

impl fmt::Display for QueryAttr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct QueryValue(String);

pub trait Primitive: Display {}
impl Primitive for u8 {}
impl Primitive for u16 {}
impl Primitive for u32 {}
impl Primitive for u64 {}
impl Primitive for i8 {}
impl Primitive for i16 {}
impl Primitive for i32 {}
impl Primitive for i64 {}
impl Primitive for f32 {}
impl Primitive for f64 {}

macro_rules! impl_primitive {
    ($ty:ty) => {
        impl From<$ty> for QueryValue {
            fn from(value: $ty) -> Self {
                QueryValue::new_primitive(value)
            }
        }
        
        impl<const N: usize> From<[$ty; N]> for QueryValue {
            fn from(value: [$ty; N]) -> Self {
                QueryValue::new_primitive_array(&value)
            }
        }
        
        impl From<Vec<$ty>> for QueryValue {
            fn from(value: Vec<$ty>) -> Self {
                QueryValue::new_primitive_array(&value)
            }
        }
    };
    ($ty:ty, $($rest:ty),+) => {
        impl_primitive!($ty);
        impl_primitive!($($rest),+);
    };
}

impl_primitive!(u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);

macro_rules! impl_string {
    ($ty:ty) => {
        impl From<$ty> for QueryValue {
            fn from(value: $ty) -> Self {
                QueryValue::new_string(value)
            }
        }
        
        impl<const N: usize> From<[$ty; N]> for QueryValue {
            fn from(value: [$ty; N]) -> Self {
                QueryValue::new_string_array(&value)
            }
        }
        
        impl From<Vec<$ty>> for QueryValue {
            fn from(value: Vec<$ty>) -> Self {
                QueryValue::new_string_array(&value)
            }
        }
    };
    ($ty:ty, $($rest:ty),+) => {
        impl_string!($ty);
        impl_string!($($rest),+);
    };
}

impl_string!(String, &str);

impl QueryValue {
    pub fn new_string<S>(value: S) -> Self
    where
        S: Into<String>,
    {
        QueryValue(format!("[\"{}\"]", value.into()))
    }

    pub fn new_primitive<P>(value: P) -> Self
    where
        P: Primitive
    {
        QueryValue(format!("[{}]", value))
    }

    pub fn new_primitive_array<P>(value: &[P]) -> Self
    where
        P: Primitive + Display,
    {
        let mut s = String::new();
        s.push('[');
        for (i, v) in value.iter().enumerate() {
            if i > 0 {
                s.push(',');
            }
            s.push_str(&v.to_string());
        }
        s.push(']');
        QueryValue(s)
    }

    pub fn new_string_array<S>(value: &[S]) -> Self
    where
        S: Display,
    {
        let mut s = String::new();
        s.push('[');
        for (i, v) in value.iter().enumerate() {
            if i > 0 {
                s.push(',');
            }
            #[allow(clippy::format_push_string)]
            s.push_str(&format!("\"{}\"", v));
        }
        s.push(']');
        QueryValue(s)
    }
}

impl fmt::Display for QueryValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub enum Query {
    Equal(QueryAttr, QueryValue),
    NotEqual(QueryAttr, QueryValue),
    LessThan(QueryAttr, QueryValue),
    LessThanEqual(QueryAttr, QueryValue),
    GreaterThan(QueryAttr, QueryValue),
    GreaterThanEqual(QueryAttr, QueryValue),
    Search(QueryAttr, QueryValue),
    OrderDesc(QueryAttr),
    OrderAsc(QueryAttr),
    Limit(u32),
    Offset(u32),
    CursorAfter(DocumentId),
    CursorBefore(DocumentId),
}

impl Display for Query {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Query::Equal(key, value) => write!(f, "equal({},{})", key, value),
            Query::NotEqual(key, value) => write!(f, "notEqual({},{})", key, value),
            Query::LessThan(key, value) => write!(f, "lessThan({},{})", key, value),
            Query::LessThanEqual(key, value) => write!(f, "lessThanEqual({},{})", key, value),
            Query::GreaterThan(key, value) => write!(f, "greaterThan({},{})", key, value),
            Query::GreaterThanEqual(key, value) => write!(f, "greaterThanEqual({},{})", key, value),
            Query::Search(key, value) => write!(f, "search({},{})", key, value),
            Query::OrderDesc(key) => write!(f, "orderDesc({})", key),
            Query::OrderAsc(key) => write!(f, "orderAsc({})", key),
            Query::Limit(limit) => write!(f, "limit({})", limit),
            Query::Offset(offset) => write!(f, "offset({})", offset),
            Query::CursorAfter(cursor) => write!(f, "cursorAfter(\"{}\")", cursor),
            Query::CursorBefore(cursor) => write!(f, "cursorBefore(\"{}\")", cursor),
        }
    }
}

pub trait QueryExt {
    fn equal<V>(&self, v: V) -> Query
    where
        V: Into<QueryValue>;
    fn not_equal<V>(&self, v: V) -> Query
    where
        V: Into<QueryValue>;
    fn less_than<V>(&self, v: V) -> Query
    where
        V: Into<QueryValue>;
    fn less_than_equal<V>(&self, v: V) -> Query
    where
        V: Into<QueryValue>;
    fn greater_than<V>(&self, v: V) -> Query
    where
        V: Into<QueryValue>;
    fn greater_than_equal<V>(&self, v: V) -> Query
    where
        V: Into<QueryValue>;
    fn search<V>(&self, v: V) -> Query
    where
        V: Into<QueryValue>;
    fn order_desc(&self) -> Query;
    fn order_asc(&self) -> Query;
}

impl QueryExt for &str {
    fn equal<V>(&self, v: V) -> Query
    where
        V: Into<QueryValue>,
    {
        let attr = QueryAttr::new(self.to_string());
        Query::Equal(attr, v.into())
    }

    fn not_equal<V>(&self, v: V) -> Query
    where
        V: Into<QueryValue>,
    {
        let attr = QueryAttr::new(self.to_string());
        Query::NotEqual(attr, v.into())
    }

    fn less_than<V>(&self, v: V) -> Query
    where
        V: Into<QueryValue>,
    {
        let attr = QueryAttr::new(self.to_string());
        Query::LessThan(attr, v.into())
    }

    fn less_than_equal<V>(&self, v: V) -> Query
    where
        V: Into<QueryValue>,
    {
        let attr = QueryAttr::new(self.to_string());
        Query::LessThanEqual(attr, v.into())
    }

    fn greater_than<V>(&self, v: V) -> Query
    where
        V: Into<QueryValue>,
    {
        let attr = QueryAttr::new(self.to_string());
        Query::GreaterThan(attr, v.into())
    }

    fn greater_than_equal<V>(&self, v: V) -> Query
    where
        V: Into<QueryValue>,
    {
        let attr = QueryAttr::new(self.to_string());
        Query::GreaterThanEqual(attr, v.into())
    }

    fn search<V>(&self, v: V) -> Query
    where
        V: Into<QueryValue>,
    {
        let attr = QueryAttr::new(self.to_string());
        Query::Search(attr, v.into())
    }

    fn order_desc(&self) -> Query {
        let attr = QueryAttr::new(self.to_string());
        Query::OrderDesc(attr)
    }

    fn order_asc(&self) -> Query {
        let attr = QueryAttr::new(self.to_string());
        Query::OrderAsc(attr)
    }
}

impl QueryExt for String {
    fn equal<V>(&self, v: V) -> Query
    where
        V: Into<QueryValue>,
    {
        let attr = QueryAttr::new(self);
        Query::Equal(attr, v.into())
    }

    fn not_equal<V>(&self, v: V) -> Query
    where
        V: Into<QueryValue>,
    {
        let attr = QueryAttr::new(self);
        Query::NotEqual(attr, v.into())
    }

    fn less_than<V>(&self, v: V) -> Query
    where
        V: Into<QueryValue>,
    {
        let attr = QueryAttr::new(self);
        Query::LessThan(attr, v.into())
    }

    fn less_than_equal<V>(&self, v: V) -> Query
    where
        V: Into<QueryValue>,
    {
        let attr = QueryAttr::new(self);
        Query::LessThanEqual(attr, v.into())
    }

    fn greater_than<V>(&self, v: V) -> Query
    where
        V: Into<QueryValue>,
    {
        let attr = QueryAttr::new(self);
        Query::GreaterThan(attr, v.into())
    }

    fn greater_than_equal<V>(&self, v: V) -> Query
    where
        V: Into<QueryValue>,
    {
        let attr = QueryAttr::new(self);
        Query::GreaterThanEqual(attr, v.into())
    }

    fn search<V>(&self, v: V) -> Query
    where
        V: Into<QueryValue>,
    {
        let attr = QueryAttr::new(self);
        Query::Search(attr, v.into())
    }

    fn order_desc(&self) -> Query {
        let attr = QueryAttr::new(self);
        Query::OrderDesc(attr)
    }

    fn order_asc(&self) -> Query {
        let attr = QueryAttr::new(self);
        Query::OrderAsc(attr)
    } 
}