use cosmwasm_schema::cw_serde;
use cw_storage_plus::{Map};

#[cw_serde]
pub struct Website {
    pub creator: String,
    pub source: String,
}

pub const WEBSITES: Map<&str, Website> = Map::new("websites");
