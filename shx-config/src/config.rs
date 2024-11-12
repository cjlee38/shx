use std::path::{Path, PathBuf};

use anyhow::{bail, Context};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub cdx_config: CdxConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            cdx_config: CdxConfig::default(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CdxConfig {
    search_size: Option<usize>,
    max_size: Option<usize>,
}

impl Default for CdxConfig {
    fn default() -> Self {
        CdxConfig {
            search_size: Some(30),
            max_size: Some(1024),
        }
    }
}

impl CdxConfig {
    pub fn search_size(&self) -> usize {
        self.search_size.unwrap_or(30)
    }

    pub fn max_size(&self) -> usize {
        self.max_size.unwrap_or(1024)
    }
}

pub fn config() -> anyhow::Result<Config> {
    let home = shx_home()?;
    let buf = home.join("config.toml");
    if buf.exists() {
        let content = std::fs::read_to_string(buf).ok();
        if content.is_none() {
            return Ok(Config::default());
        }
        let content = content.unwrap();
        if content.is_empty() {
            return Ok(Config::default());
        }
        return toml::from_str::<Config>(&content).context("[fatal] failed to parse config file");
    }
    // TODO : initialize config file
    bail!("Cannot find config file")
}

pub fn path_for<P>(name: P) -> anyhow::Result<PathBuf>
where
    P: AsRef<Path>,
{
    if let Ok(shx_home) = shx_home() {
        return Ok(shx_home.join(name));
    }
    bail!("[error] Cannot find directory from $SHX_HOME {}", name.as_ref().display())
}

fn shx_home() -> anyhow::Result<PathBuf> {
    if let Ok(shx_home) = std::env::var("SHX_HOME") {
        return Ok(PathBuf::from(shx_home));
    }
    if let Ok(home) = home() {
        return Ok(PathBuf::from(home).join(".shx"));
    }
    bail!("[error] Cannot find shx home directory")
}

pub fn home() -> anyhow::Result<PathBuf> {
    if let Ok(home) = std::env::var("HOME") {
        if !home.is_empty() {
            return Ok(PathBuf::from(home));
        }
    }

    #[cfg(target_family = "windows")]
    if let Ok(profile) = std::env::var("USERPROFILE") {
        return Ok(PathBuf::from(profile));
    }

    if let Some(user) = get_current_username() {
        #[cfg(target_family = "unix")]
        {
            return Ok(PathBuf::from(format!("/home/{}", user)));
        }
        #[cfg(target_family = "windows")]
        {
            return Some(PathBuf::from(format!("C:\\Users\\{}", user)));
        }
    }
    bail!("[fatal] Cannot find home directory")
}

fn get_current_username() -> Option<String> {
    if let Ok(user) = std::env::var("USER") {
        return Some(user);
    }
    if let Ok(user) = std::env::var("USERNAME") {
        return Some(user);
    }

    #[cfg(target_family = "unix")]
    {
        use std::process::Command;
        if let Ok(output) = Command::new("whoami").output() {
            if let Ok(username) = String::from_utf8(output.stdout) {
                return Some(username.trim().to_string());
            }
        }
    }

    None
}