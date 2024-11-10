use std::path::PathBuf;

// TODO : Implement Config when needed.
pub struct Config {}

pub fn config() {
    println!("config");
    if let Some(home) = shx_home() {
        let buf = home.join("config.toml");
        if buf.exists() {
            println!("{:?} config exists", buf);
        } else {
            println!("{:?} config not exists", buf);
        }
    }
}

pub fn path_for(name: &str) -> Option<PathBuf> {
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