use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub docker: DockerConfig,
    pub containers: HashMap<String, String>,
    pub languages: HashMap<String, LanguageConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub bind_addr: String,
    pub run_timeout: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DockerConfig {
    pub host: String,
    pub connect_timeout: u64,
    pub keepalive_secs: u64,
    pub request_timeout: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LanguageConfig {
    pub enabled: bool,
    pub file_extension: String,
    // #[serde(default)]
    // pub compiler: Option<String>,
    #[serde(default)]
    pub compile_cmd: Option<String>,
    pub run_cmd: String
}


impl Config {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn get_language(&self, lang: &str) -> Option<&LanguageConfig> {
        self.languages.get(lang).filter(|cfg| cfg.enabled)
    }

    pub fn get_container(&self, lang: &str) -> Option<&str> {
        self.containers.get(lang).map(|s| s.as_str())
    }
}