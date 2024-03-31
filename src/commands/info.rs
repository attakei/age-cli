/* 'info' command displays data from config-file.
 */
use crate::config::Config;
use crate::config::DEFAULT_FILENAME;
use crate::versioning;

use anyhow::Result;
use clap::Args;
use tera::{Context, Tera};

#[derive(Args)]
pub(crate) struct Arguments {}

pub(crate) fn execute(_args: &Arguments, config: &Config) -> Result<()> {
    let mut ctx = Context::new();
    let mut files: Vec<String> = Vec::new();
    files.push(DEFAULT_FILENAME.to_string());
    ctx.insert("current_version", &config.current_version);
    ctx.insert("next_major", &versioning::up_major(&config.current_version));
    ctx.insert("next_minor", &versioning::up_minor(&config.current_version));
    ctx.insert("next_patch", &versioning::up_patch(&config.current_version));
    for f in config.get_files() {
        files.push(f.path.display().to_string());
    }
    ctx.insert("files", &files);
    // Display information data.
    println!("{}", Tera::one_off(DISPLAY_TEMPLATE, &ctx, true).unwrap());
    Ok(())
}

const DISPLAY_TEMPLATE: &'static str = r#"
# Version info

- Current:    {{ current_version }}
- Next major: {{ next_major }}
- Next minor: {{ next_minor }}
- Next patch: {{ next_patch }}

# Replace targets

{% for f in files -%}
- {{ f|safe }}
{% endfor -%}
"#;
