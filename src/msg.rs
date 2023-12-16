use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;

#[cw_serde]
pub struct InstantiateMsg {
    pub managers: Option<Vec<String>>,

    pub cost: Option<Coin>,

    /// per_period is the amount of blocks that the cost is valid for
    /// if None, the site is hosted forever at the original cost (initialization fee)
    /// if per_period is Some(), the site cost is required for every per_period blocks
    ///
    /// i.e. if per_period is Some(100), the site cost is required every 100 blocks. Preload your account with funds
    pub period: Option<u64>,

    /// false for non x/clock module chains (SudoMsg).
    pub using_clock: bool,
}

#[cw_serde]
pub enum ExecuteMsg {
    NewWebsite { name: String, source: String },
    ExpireOld {},

    // admin only
    Remove { name: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(crate::state::Website)]
    GetWebsite { name: String },
}
