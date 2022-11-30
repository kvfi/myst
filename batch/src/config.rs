use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use log::{error, info};
use serde::{Deserialize, Serialize};

pub static CONFIG_FILE_PATH: &str = "./config.json";

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SmtpCredentials {
    pub(crate) server: String,
    pub(crate) username: String,
    pub(crate) password: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub(crate) consumer_key: String,
    pub(crate) redirect_url: String,
    pub(crate) api_endpoint: String,
    pub(crate) batch_schedule: String,
    pub(crate) database_url: String,
    pub(crate) code: Option<String>,
    pub(crate) token: Option<String>,
    pub(crate) auth_url: Option<String>,
    pub(crate) code_valid: Option<bool>,
    pub(crate) debug: Option<bool>,
    pub(crate) last_retrieval: Option<i64>,
    pub(crate) smtp_credentials: Option<SmtpCredentials>,
}

impl Config {
    pub fn save(&mut self) {
        let c = serde_json::to_string_pretty(&self).unwrap();
        let mut file = File::create(Path::new(CONFIG_FILE_PATH)).unwrap();
        match file.write_all(c.as_bytes()) {
            Ok(_) => {
                info!("Updated config written to disk")
            }
            Err(e) => {
                error!("Cannot write to file: {:?}", e);
            }
        }
    }
}

pub fn get_config(path: Option<String>) -> Config {
    let cfg_path = match path {
        Some(p) => p,
        None => CONFIG_FILE_PATH.to_string(),
    };

    let config_filepath = Path::new(&cfg_path);
    let config_content =
        fs::read_to_string(config_filepath).expect("Something went wrong reading the file");
    let config: Config = serde_json::from_str(&*config_content).unwrap();
    config
}
