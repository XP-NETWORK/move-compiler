#[macro_export]
macro_rules! TEMPLATE_CHILD_ACC_CREATE {
    ($coint:tt, $($args:tt)*) => {
        fmt!(
            TEMPLATE_SCRIPT_MAIN,
            imports = format!(r#"
    use 0x1::{coin_type};
    use 0x1::AccountCreationScripts;
            "#, coin_type = $coint),
            main_args = "s: signer",
            main_body = fmt!("AccountCreationScripts::create_child_vasp_account<{coin_type}>(s, {child_address}, {auth_key_prefix}, {all_currencies}, {initial_bal});", coin_type = $coint, $($args)*)
        )
    }
}