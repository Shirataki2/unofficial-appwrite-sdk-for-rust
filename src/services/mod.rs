pub mod accounts;
pub mod avatars;
pub mod databases;
pub mod functions;
pub mod health;
pub mod locales;
pub mod storages;
pub mod teams;
pub mod users;

#[derive(Debug, Display, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CursorDirection {
    #[display(fmt = "before")]
    Before,
    #[display(fmt = "after")]
    After,
}

#[derive(Debug, Display, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Order {
    #[display(fmt = "asc")]
    Asc,
    #[display(fmt = "desc")]
    Desc,
}

#[derive(Debug, Clone, Default, SerializeParams)]
pub struct SearchPayload<ID> {
    pub seatch: Option<String>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub cursor: Option<ID>,
    pub cursor_direction: Option<CursorDirection>,
    pub order_type: Option<Order>,
}

#[derive(Debug, Clone, Default, SerializeParams)]
pub struct SearchQueryPayload<ID> {
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub cursor: Option<ID>,
    pub cursor_direction: Option<CursorDirection>,
    pub order_attribute: Option<String>,
    pub order_type: Option<Order>,
}
