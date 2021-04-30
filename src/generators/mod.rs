use crate::*;
use reusable_fmt::fmt;

pub fn child_account_creation(
    coin_type: &str,
    child_address: u128,
    auth_key_prefix: &str,
    all_currencies: bool,
    initial_bal: u64,
) -> String {
    TEMPLATE_CHILD_ACC_CREATE!(
        coin_type,
        child_address = child_address,
        auth_key_prefix = auth_key_prefix,
        all_currencies = all_currencies,
        initial_bal = initial_bal
    )
}

pub fn payment_p2p(
    coin_type: &str,
    receiver: u128,
    amount: u64,
    meta_hex: Option<&str>,
    meta_sig_hex: Option<&str>,
) -> String {
    let mut b1 = String::new();
    let mut b2 = String::new();

    let metadata = meta_hex
        .map(|h| {
            b1 = format!("x\"{}\"", h);
            b1.as_str()
        })
        .unwrap_or("Vector::empty<u8>()");
    let meta_sig = meta_sig_hex
        .map(|h| {
            b2 = format!("x\"{}\"", h);
            b2.as_str()
        })
        .unwrap_or("Vector::empty<u8>()");

    TEMPLATE_PAYMENT_P2P!(
        coin_type,
        receiver = receiver,
        amount = amount,
        metadata = metadata,
        meta_sig = meta_sig
    )
}
