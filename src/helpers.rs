
use crate::errors;

use bcs::from_bytes;
use diem_types::transaction::SignedTransaction;
use hex::FromHex;

/// Create a parent diem account
pub async fn create_parent_account(
    client: &mut reqwest::Client,
    faucet: &str,
    amount: u128,
    auth_key: &str,
    cc: &str,
) -> Result<Vec<SignedTransaction>, errors::AccountCreationError> {
    let res = client
        .post(faucet)
        .query(&[("amount", amount.to_string())])
        .query(&[("auth_key", auth_key)])
        .query(&[("currency_code", cc)])
        .query(&[("return_txns", true)])
        .send()
        .await?;

    let res_t = res.text().await?;
    let bytes = Vec::from_hex(res_t).unwrap();

    Ok(from_bytes::<Vec<SignedTransaction>>(bytes.as_slice()).unwrap())
}