pub mod account_creation;
pub mod payments;

use reusable_fmt::fmt_reuse;

// Template for a default Move script
fmt_reuse! {
    TEMPLATE_SCRIPT_MAIN = r#"
script {{
{imports}

    fun main({main_args}) {{
        {main_body}
    }}
}}
"#;
}