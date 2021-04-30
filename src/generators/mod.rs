use crate::*;
use reusable_fmt::fmt;

pub fn child_account_creation(
    coin_type: &str,
    child_address: &str,
    auth_key_prefix: &str, 
    all_currencies: bool,
    initial_bal: u128) -> String {
    TEMPLATE_CHILD_ACC_CREATE!(
        coin_type,
        child_address = child_address,
        auth_key_prefix = auth_key_prefix,
        all_currencies = all_currencies,
        initial_bal = initial_bal)
}