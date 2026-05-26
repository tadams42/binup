use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Default)]
struct RelgetConfig {
    github_token:   Option<String>,
    codeberg_token: Option<String>,
    gitlab_token:   Option<String>,
}

fn load_config() -> Result<RelgetConfig> {
    let path = dirs::config_dir().unwrap_or_default().join("relget.toml");
    if !path.exists() {
        return Ok(RelgetConfig::default());
    }
    Ok(toml::from_str(&std::fs::read_to_string(&path)?)?)
}

pub fn load_github_token() -> Result<Option<String>> {
    if let Ok(t) = std::env::var("RELGET_GHB_TOKEN") {
        if !t.is_empty() {
            return Ok(Some(t));
        }
    }
    Ok(load_config()?.github_token)
}

pub fn load_codeberg_token() -> Result<Option<String>> {
    if let Ok(t) = std::env::var("RELGET_CDB_TOKEN") {
        if !t.is_empty() {
            return Ok(Some(t));
        }
    }
    Ok(load_config()?.codeberg_token)
}

pub fn load_gitlab_token() -> Result<Option<String>> {
    if let Ok(t) = std::env::var("RELGET_GLB_TOKEN") {
        if !t.is_empty() {
            return Ok(Some(t));
        }
    }
    Ok(load_config()?.gitlab_token)
}
