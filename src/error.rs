use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Must send '{0}' to deposit")]
    MissingDenom(String),

    #[error("Sent unsupported denoms, must send '{0}' to deposit")]
    ExtraDenoms(String),    

    #[error("No funds sent")]
    NoFunds {},

    #[error("Must send valid address to deposit")]
    InvalidDenom(String),

    #[error("Missed address or denom")]
    MixedNativeAndCw20(String),    
}
