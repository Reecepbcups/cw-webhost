use cosmwasm_schema::cw_serde;
use cosmwasm_std::Coin;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Website {
    pub creator: String,
    pub source: String,

    /// created_height is useful to trust the length of time a domain is up for
    /// and also if the manager decides later to add a cooldown period from infinite before.
    pub created_height: u64,

    pub in_grace_period: bool,
}
pub const WEBSITES: Map<&str, Website> = Map::new("ws");

// block height, domain names that expire at that height (moved to temp grace period)
#[cw_serde]
pub struct ExpireTimes {
    pub names: Vec<String>,
}
pub const EXPIRE_TIMES: Map<u64, ExpireTimes> = Map::new("et");
pub const GRACE_PERIOD: Map<u64, ExpireTimes> = Map::new("g");

#[cw_serde]
pub struct Config {
    pub manager: Option<Vec<String>>,
    // cost for the domain per period
    pub cost: Option<Coin>,
    /// per_period is the amount of blocks that the cost for the site is valid for.
    /// 0 is infinite
    pub period: u64,
}
pub const CONFIG: Item<Config> = Item::new("c");

#[cw_serde]
pub struct ShortLink {
    pub creator: String,
    pub original_url: String,
    pub created_height: u64,
}

pub const SHORT_LINKS: Map<&str, ShortLink> = Map::new("sl");