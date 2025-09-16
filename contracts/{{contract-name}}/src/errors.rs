use crate::*;

#[derive(thiserror::Error, Debug)]
pub enum GlobalError {
    #[error("Invalid json: {0}")]
    InvalidJson(near_sdk::serde_json::Error),
    #[error("State error: Cannot load in contract due to missing state")]
    ContractStateIsMissing,
}

#[derive(thiserror::Error, Debug)]
pub enum ContractError {
    #[error("{0}.")]
    Global(#[from] GlobalError),
}

impl near_sdk::FunctionError for ContractError {
    fn panic(&self) -> ! {
        env::panic_str(&self.to_string())
    }
}
