use std::{fs, str};

use serde_yaml;
use serde::Deserialize;
use std::collections::HashMap;

fn format_address(host: &str, port: &str) -> String {
    format!("{}:{}", host, port)
}

#[derive(Debug, Deserialize)]
pub struct HttpConfig {
    pub host: String,
    pub port: String,
}

#[derive(Debug, Deserialize)]
pub struct SslConfig {
    pub enabled: bool,
    pub host: String,
    pub port: String,
    pub cert_path: String,
    pub key_path: String,
}

#[derive(Debug, Deserialize)]
pub struct RuntimeConfig {
    pub workers: usize,
}

#[derive(Debug, Deserialize)]
pub struct FileConfig {
    pub in_memory: bool,
    pub path: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ContentConfig {
    pub root_path: String,
    pub files: HashMap<String, FileConfig>,
}


#[derive(Debug, Deserialize)]
pub struct Config {
    pub http: HttpConfig,
    pub ssl: SslConfig,
    pub runtime: RuntimeConfig,
    pub content: ContentConfig,
}

impl Config {
    pub fn parse(config_str: &str) -> Result<Config, String> {
        match serde_yaml::from_str(config_str) {
            Ok(parsed_config) => Ok(parsed_config),
            Err(err) => Err(err.to_string()),
        }
    }

    pub fn get_http_address(&self) -> String {
        format_address(&self.http.host, &self.http.port)
    }

    pub fn get_ssl_address(&self) -> String {
        format_address(&self.ssl.host, &self.ssl.port)
    }

    pub fn from_file(file_path: &str) -> Result<Config, String> {
        let config_content = match fs::read(file_path) {
            Ok(content) => content,
            Err(err) => {
                let err_msg = format!("Can not read config: {}", err.to_string());
                return Err(err_msg);
            }
        };

        match Config::parse(str::from_utf8(&config_content).unwrap()) {
            Ok(config) => Ok(config),
            Err(err) => {
                let err_msg = format!("Config is invalid: {}", err.to_string());
                Err(err_msg)
            },
        }
    }
}
