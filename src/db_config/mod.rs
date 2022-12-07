use std::fs::File;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Development {
    pub address: String,
    pub port: String,
    pub workers: u8,
    pub database: Database,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub adapter: String,
    pub db_name: String,
    pub pool: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub development: Development,
}

pub fn read_config(file_path: String) -> Config {
    let f = File::open(file_path).expect("===open file error===");
    serde_json::from_reader(f).unwrap()
}

