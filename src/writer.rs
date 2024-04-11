use std::collections::{HashMap, VecDeque};
use std::fs::{read_to_string, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use anyhow::Result;
use tera::{Context, Tera};

use crate::config::FileConfig;

/**
 * File writer.
 */
pub struct Writer {
    context: Context,
    targets: HashMap<String, WriteTarget>,
}

/**
 * Writing target context per files.
 */
pub struct WriteTarget {
    /** Target filepath. */
    path: PathBuf,
    /** Replacement ruleset. */
    rules: Vec<WriteRule>,
}

/**
 * Replacement rule component.
 *
 * Writing proc refer 'search' and 'replace' as pair.
 */
pub struct WriteRule {
    /** Search target that is rendered by Tera. */
    pub search: String,
    /** Replacement content that is rendered by Tera. */
    pub replace: String,
    pub regex: bool,
}

impl Writer {
    pub fn new(ctx: &Context) -> Self {
        Self {
            context: ctx.clone(),
            targets: HashMap::new(),
        }
    }

    pub fn add_target(&mut self, config: &FileConfig) {
        let path_key = config.path.display().to_string();
        if !self.targets.contains_key(&path_key) {
            self.targets
                .insert(path_key.clone(), WriteTarget::new(&config.path));
        }
        let target = self.targets.get_mut(&path_key).unwrap();
        let rule = WriteRule {
            search: Tera::one_off(&config.search, &self.context, true)
                .unwrap()
                .to_string(),
            replace: Tera::one_off(&config.replace, &self.context, true)
                .unwrap()
                .to_string(),
            regex: config.regex(),
        };
        target.add_rule(rule);
    }

    pub fn update_all(&self) -> Result<()> {
        for t in self.targets.values() {
            t.update().unwrap();
        }
        Ok(())
    }
}

impl WriteTarget {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
            rules: Vec::new(),
        }
    }

    pub fn update(&self) -> Result<()> {
        let mut text = read_to_string(&self.path).unwrap();
        for rule in &self.rules {
            text = rule.update(text);
        }
        let mut out = File::create(&self.path)?;
        let _ = out.write(text.as_bytes());
        Ok(())
    }

    pub fn add_rule(&mut self, rule: WriteRule) {
        self.rules.push(rule);
    }
}

impl WriteRule {
    fn update(&self, target: String) -> String {
        let lines = self.search.split('\n').count();
        let mut buf: VecDeque<String> = VecDeque::new();
        let mut output: Vec<String> = Vec::new();
        for line in target.split('\n') {
            if buf.len() == lines {
                output.push(buf.pop_front().unwrap());
            }
            buf.push_back(line.to_string());
            let buf_vec: Vec<String> = buf.clone().into();
            if buf_vec.join("\n") == self.search {
                output.push(self.replace.to_string());
                buf.clear();
            }
        }
        if !buf.is_empty() {
            for line in buf {
                output.push(line.to_string());
            }
        }
        output.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use semver::Version;

    use super::*;

    #[test]
    fn new_writer() {
        let ctx = Context::new();
        let writer = Writer::new(&ctx);
        assert_eq!(writer.targets.len(), 0);
    }

    #[test]
    fn dual_rule_in_single_file() {
        let mut ctx = Context::new();
        ctx.insert("current_version", &Version::new(0, 1, 0));
        ctx.insert("new_version", &Version::new(0, 2, 0));
        let mut writer = Writer::new(&ctx);
        let filepath = PathBuf::from("dummy.txt");
        writer.add_target(&FileConfig {
            path: filepath.clone(),
            search: String::from("target-1"),
            replace: String::from("replace-2"),
            regex: Some(false),
        });
        writer.add_target(&FileConfig {
            path: filepath.clone(),
            search: String::from("target-2"),
            replace: String::from("replace-2"),
            regex: Some(false),
        });
        assert_eq!(writer.targets.len(), 1);
    }
}
