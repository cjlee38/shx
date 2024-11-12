use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, Context};
use serde::{Deserialize, Serialize};

use crate::cdx::CdxConfig;

#[derive(Debug, Serialize, Deserialize)]
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

pub fn config() -> anyhow::Result<Config> {
    let home = shx_home()?;
    let config_path = home.join("config.toml");
    if config_path.exists() {
        let content = fs::read_to_string(&config_path).ok();
        if content.is_none() {
            return fallback(config_path);
        }
        let content = content.unwrap();
        if content.is_empty() {
            return fallback(config_path);
        }
        // TODO : detect which caused parsing error & tell user.
        return toml::from_str::<Config>(&content).context("[fatal] failed to parse config file");
    }
    fallback(config_path)
}

/// Creates a default config file when missing
fn fallback<P>(path: P) -> anyhow::Result<Config>
where
    P: AsRef<Path>,
{
    let config = Config::default();

    toml::to_string(&config)
        .map_err(|e| anyhow!("[fatal] failed to serialize config: {}", e))
        .and_then(|serialized| fs::write(path, serialized).map_err(|e| anyhow!("[fatal] failed to write config: {}", e)))
        .map(|_| config)
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
    let home = home()?;
    let shx_home = PathBuf::from(home).join(".shx");
    // ensure shx_home exists
    if !shx_home.exists() {
        fs::create_dir_all(&shx_home)
            .context(format!("[fatal] failed to create $SHX_HOME {}", shx_home.display()))?;
    }
    Ok(shx_home)
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