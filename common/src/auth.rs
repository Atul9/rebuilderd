use crate::errors::*;
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub auth: AuthConfig,
}

#[derive(Debug, Default, Deserialize)]
pub struct AuthConfig {
    pub cookie: Option<String>,
}

fn read_cookie_from_config<P: AsRef<Path>>(path: P) -> Result<Option<String>> {
    if let Ok(buf) = fs::read(path) {
        let config = toml::from_slice::<Config>(&buf)?;
        Ok(config.auth.cookie)
    } else {
        Ok(None)
    }
}

fn read_cookie_from_file<P: AsRef<Path>>(path: P) -> Result<String> {
    let cookie = fs::read_to_string(path)?;
    Ok(cookie.trim().to_string())
}

pub fn find_auth_cookie() -> Result<String> {
    if let Ok(cookie_path) = env::var("REBUILDERD_COOKIE_PATH") {
        return read_cookie_from_file(cookie_path);
    }

    if let Some(config_dir) = dirs::config_dir() {
        let path = config_dir.join("rebuilderd.conf");
        if let Some(cookie) = read_cookie_from_config(path)? {
            return Ok(cookie);
        }
    }

    if let Some(cookie) = read_cookie_from_config("/etc/rebuilderd.conf")? {
        return Ok(cookie);
    }

    if let Ok(cookie) = read_cookie_from_file("/var/lib/rebuilderd/auth-cookie") {
        return Ok(cookie);
    }

    if let Some(data_dir) = dirs::data_dir() {
        let path = data_dir.join("rebuilderd-auth-cookie");
        return read_cookie_from_file(path);
    }

    bail!("Failed to find auth cookie anywhere")
}