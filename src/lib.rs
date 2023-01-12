#![allow(clippy::unit_arg)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate derive_more;

#[macro_use]
extern crate param_macro;

pub mod client;
pub mod error;
pub mod macros;
pub mod models;
pub mod services;

pub use attr_macro::AppWriteModel;
use prelude::DeploymentId;
use serde::Deserialize;

pub mod prelude {
    pub use super::AppWriteModel;
    pub use crate::client::AppWriteClient;
    pub use crate::error::Error;
    pub use crate::models::prelude::*;
    pub use crate::services::{
        accounts::*, avatars::*, databases::*, functions::*, health::*, locales::*, storages::*,
        teams::*, users::*, CursorDirection, Order, SearchPayload, SearchQueryPayload,
    };
}

pub(crate) fn empty_deploy_as_none<'de, D>(
    deserializer: D,
) -> Result<Option<DeploymentId>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        Ok(Some(DeploymentId::new(s)))
    }
}
