use crate::*;

const TEST_FAUCET: &str = "http://faucet.testnet.diem.com";
const PARENT_ACC_ENDPOINT: &str = "/mint";
const TEST_CURRENCY: &str = "XDX";
const TEST_KEY: &str = "9b23fa936404bb467d78b9f361837c4d39dddd1b74302fb4a4af872937cfa1ff";

#[tokio::test]
async fn test_parent_account_creation() {
    let mut client = reqwest::Client::new();

    assert!(helpers::create_parent_account(
        &mut client,
        &format!("{}{}", TEST_FAUCET, PARENT_ACC_ENDPOINT),
        1,
        TEST_KEY,
        TEST_CURRENCY
    )
    .await
    .is_ok());
}

#[cfg(feature = "test_generated")]
#[test]
fn test_child_account_creation_generate() {
    println!(
        "{}",
        generators::child_account_creation("XDX", 0x1, "xdg", false, 0)
    );
}

#[cfg(feature = "test_generated")]
#[test]
fn test_transfer_p2p() {
    println!("{}", generators::payment_p2p("XDX", 0x0, 32, None, None));
}
