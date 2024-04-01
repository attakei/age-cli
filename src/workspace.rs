use anyhow::Result;
use chrono::Local;
use semver::Version;
use std::path::PathBuf;
use tera::Context;

use crate::config::{resolve_config, Config, DEFAULT_FILENAME};
use crate::writer::Writer;

/**
 * CLI workspace.
 *
 * .. note:: Under construction
 */
#[derive(Debug)]
pub struct Workspace {
    pub config: Config,
}

impl Workspace {
    pub fn try_new(root: PathBuf) -> Result<Self> {
        let resolved = resolve_config(&root);
        if resolved.is_err() {
            return Err(resolved.unwrap_err());
        }
        Ok(Self {
            config: resolved.unwrap(),
        })
    }

    #[allow(dead_code)]
    fn init_writer(&self, ctx: &Context) -> Writer {
        let mut writer = Writer::new(ctx);
        writer.add_target(
            &PathBuf::from(DEFAULT_FILENAME),
            &("current_version = \"{{current_version}}\"".to_string()),
            &("current_version = \"{{new_version}}\"".to_string()),
        );
        for f in &self.config.files {
            writer.add_target(&f.path, &f.search, &f.replace);
        }
        writer
    }

    #[allow(dead_code)]
    pub fn update_files(&self, ctx: &Context) -> Result<()> {
        let writer = self.init_writer(ctx);
        match writer.update_all() {
            Ok(_) => {
                println!("Updated!");
                Ok(())
            }
            Err(err) => Err(err),
        }
    }
}

pub fn make_context(current_version: &Version, new_version: &Version) -> Context {
    let mut ctx = Context::new();
    ctx.insert("current_version", current_version);
    ctx.insert("new_version", new_version);
    ctx.insert("now", &Local::now().to_rfc3339());
    return ctx;
}
