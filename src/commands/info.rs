/* 'info' command displays data from config-file.
 */
use anyhow::Result;
use clap::Args;
use tera::{Context, Tera};

use crate::versioning;
use crate::workspace::Workspace;

#[derive(Args)]
pub(crate) struct Arguments {}

pub(crate) fn execute(_args: &Arguments, workspace: &Workspace) -> Result<()> {
    let mut ctx = Context::new();
    let mut files: Vec<String> = Vec::new();
    ctx.insert("workspace_root", &workspace.root);
    ctx.insert("workspace_file", &workspace.doc.filename());
    ctx.insert("current_version", &workspace.config.current_version);
    ctx.insert(
        "next_major",
        &versioning::up_major(&workspace.config.current_version),
    );
    ctx.insert(
        "next_minor",
        &versioning::up_minor(&workspace.config.current_version),
    );
    ctx.insert(
        "next_patch",
        &versioning::up_patch(&workspace.config.current_version),
    );
    for f in &workspace.config.files {
        files.push(f.path.display().to_string());
    }
    ctx.insert("files", &files);
    // Display information data.
    println!("{}", Tera::one_off(DISPLAY_TEMPLATE, &ctx, true).unwrap());
    Ok(())
}

const DISPLAY_TEMPLATE: &'static str = r#"
# Workspace

- Root: {{ workspace_root|safe }}
- File: {{ workspace_file|safe }}

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
