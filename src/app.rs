use anyhow::Result;
use chrono::Local;
use semver::Version;
use std::collections::VecDeque;
use std::fs::{read_to_string, File};
use std::io::prelude::*;
use std::path::PathBuf;
use tera::{Context, Tera};

use crate::config::{Config, FileConfig, DEFAULT_FILENAME};

pub fn update(base_config: &Config, new_version: &Version) -> Result<()> {
    let mut new_config = base_config.clone();
    new_config.current_version = new_version.clone();

    // Update targets
    let current_version = &base_config.current_version;
    let ctx = make_context(current_version, new_version);
    let mut files = new_config.get_files().clone();
    files.push(FileConfig {
        path: PathBuf::from(DEFAULT_FILENAME),
        search: "current_version = \"{{current_version}}\"".to_string(),
        replace: "current_version = \"{{new_version}}\"".to_string(),
    });
    for f in files {
        let search_text = Tera::one_off(&f.search, &ctx, true).unwrap().to_string();
        let replace_text = Tera::one_off(&f.replace, &ctx, true).unwrap().to_string();
        let new_text = build_updates(read_to_string(&f.path).unwrap(), search_text, replace_text);
        let mut out = File::create(&f.path)?;
        let _ = out.write(new_text.as_bytes());
    }
    println!("Updated!!");
    Ok(())
}

fn make_context(current_version: &Version, new_version: &Version) -> Context {
    let mut ctx = Context::new();
    ctx.insert("current_version", current_version);
    ctx.insert("new_version", new_version);
    ctx.insert("now", &Local::now().to_rfc3339());
    ctx
}

fn build_updates(input: String, search_text: String, replace_text: String) -> String {
    let lines = search_text.split("\n").count();
    let mut buf: VecDeque<String> = VecDeque::new();
    let mut output: Vec<String> = Vec::new();
    for line in input.split("\n") {
        if buf.len() == lines {
            output.push(buf.pop_front().unwrap());
        }
        buf.push_back(line.to_string());
        let buf_vec: Vec<String> = buf.clone().into();
        if buf_vec.join("\n") == search_text {
            output.push(replace_text.to_string());
            buf.clear();
        }
    }
    if buf.len() > 0 {
        for line in buf {
            output.push(line.to_string());
        }
    }
    return output.join("\n");
}
