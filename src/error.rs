use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Never {}

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Not enough funds, required: {required}")]
    NotEnoughFunds { required: String },

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Website name '{name}' already exists")]
    AlreadyExists { name: String },
}
