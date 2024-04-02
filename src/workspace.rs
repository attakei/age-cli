use anyhow::{anyhow, Result};
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
    pub root: PathBuf,
    pub config: Config,
}

impl Workspace {
    pub fn try_new(root: PathBuf) -> Result<Self> {
        let resolved = resolve_config(&root);
        if resolved.is_err() {
            return Err(resolved.unwrap_err());
        }
        Ok(Self {
            root,
            config: resolved.unwrap(),
        })
    }

    /**
     * Search workspace directory upper in order to find git-repo or os-root.
     */
    pub fn find(root: PathBuf) -> Result<Self> {
        let mut pwd = root.clone();
        loop {
            let ws = Self::try_new(pwd.clone());
            if ws.is_ok() {
                return Ok(ws.unwrap());
            }
            if pwd.join(".git").exists() {
                break;
            }
            let parent = pwd.parent();
            if parent.is_none() {
                break;
            }
            pwd = parent.unwrap().to_path_buf();
        }
        Err(anyhow!("Workspace is not found."))
    }

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
