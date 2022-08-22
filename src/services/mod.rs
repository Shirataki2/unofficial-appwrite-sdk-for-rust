pub mod accounts;
pub mod databases;
pub mod functions;
pub mod storages;
pub mod teams;
pub mod users;

#[derive(Debug, Display, Clone, Serialize, Deserialize)]
pub enum CursorDirection {
    #[display(fmt = "before")]
    Before,
    #[display(fmt = "after")]
    After,
}

#[derive(Debug, Display, Clone, Serialize, Deserialize)]
pub enum OrderType {
    #[display(fmt = "ASC")]
    Asc,
    #[display(fmt = "DESC")]
    Desc,
}

#[derive(Debug, Clone, Default, SerializeParams)]
pub struct SearchPayload<ID> {
    pub seatch: Option<String>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub cursor: Option<ID>,
    pub cursor_direction: Option<CursorDirection>,
    pub order_type: Option<OrderType>,
}

#[derive(Debug, Clone, Default, SerializeParams)]
pub struct SearchQueryPayload<ID> {
    // TODO: add query model
    pub query: Option<String>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub cursor: Option<ID>,
    pub cursor_direction: Option<CursorDirection>,
    pub order_attribute: Option<String>,
    pub order_type: Option<OrderType>,
}
