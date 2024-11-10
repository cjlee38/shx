use std::path::{Path, PathBuf};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    cdx_config: CdxConfig,
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
    pub search_size: usize,
    pub max_size: usize,
}

impl Default for CdxConfig {
    fn default() -> Self {
        CdxConfig {
            search_size: 30,
            max_size: 1024,
        }
    }
}

pub fn config() -> Option<Config> {
    if let Some(home) = shx_home() {
        let buf = home.join("config.toml");
        if buf.exists() {
            let content = std::fs::read_to_string(buf).ok();
            if content.is_none() {
                return Some(Config::default());
            }
            let content = content.unwrap();
            if content.is_empty() {
                return Some(Config::default());
            }
            return toml::from_str::<Config>(&content).ok();
        }
    }
    None
}

pub fn path_for<P>(name: P) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    if let Some(shx_home) = shx_home() {
        return Some(shx_home.join(name));
    }
    None
}

fn shx_home() -> Option<PathBuf> {
    if let Ok(shx_home) = std::env::var("SHX_HOME") {
        return Some(PathBuf::from(shx_home));
    }
    if let Some(home) = home() {
        return Some(PathBuf::from(home).join(".shx"));
    }
    None
}

fn home() -> Option<PathBuf> {
    if let Ok(home) = std::env::var("HOME") {
        if !home.is_empty() {
            return Some(PathBuf::from(home));
        }
    }

    #[cfg(target_family = "windows")]
    if let Ok(profile) = std::env::var("USERPROFILE") {
        return Some(PathBuf::from(profile));
    }

    if let Some(user) = get_current_username() {
        #[cfg(target_family = "unix")]
        {
            return Some(PathBuf::from(format!("/home/{}", user)));
        }
        #[cfg(target_family = "windows")]
        {
            return Some(PathBuf::from(format!("C:\\Users\\{}", user)));
        }
    }
    None
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