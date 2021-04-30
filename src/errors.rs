use thiserror::Error;
use hex::FromHexError;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("`{0}`")]
    FromHex(#[from] FromHexError),
    #[error("`{0}`")]
    Bcs(#[from] bcs::Error)
}

#[derive(Error, Debug)]
pub enum AccountCreationError {
    #[error("`{0}`")]
    Reqwest(#[from] reqwest::Error),
    #[error("`{0}`")]
    Parse(#[from] ParseError)
}