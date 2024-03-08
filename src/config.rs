// Configuration manager.

pub const DEFAULT_FILENAME: &'static str = ".gazer.toml";
pub const DEFAULT_VALUES: &'static str = r#"
current_version = "0.0.0"

[[files]]
path = "Cargo.toml"
search = "version = \"{current_version}\""
replace = "version = \"{new_version}\""
"#;
