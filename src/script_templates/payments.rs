#[macro_export]
macro_rules! TEMPLATE_PAYMENT_P2P {
    ($coint:tt, $($args:tt)*) => {
        fmt!(
            TEMPLATE_SCRIPT_MAIN,
            imports = format!(r#"
    use 0x1::{coin_type};
    use 0x1::PaymentScripts;
    use 0x1::Vector;
            "#, coin_type = $coint),
            main_args = "s: signer",
            main_body = fmt!("PaymentScripts::peer_to_peer_with_metadata<{coin_type}>(s, {receiver}, {amount}, {metadata}, {meta_sig});", coin_type = $coint, $($args)*)
        )
    }
}
