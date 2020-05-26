extern crate serde_json;

pub struct Config {
    pub config: std::string::String,
    pub config_json: serde_json::Value,
}
use serde_json::{Result, Value};

impl Config {
    pub fn load_config(&mut self, filename: &str) -> bool {
        use std::path::Path;
        if !Path::new(filename).exists() {
            return false;
        };

        use std::fs;
        self.config = fs::read_to_string(filename).expect("Something went wrong reading the file");
        true
    }

    pub fn parse_json_from_config(&mut self) -> Result<()> {
        self.config_json = serde_json::from_str(&self.config)?;

        Ok(())
    }

    pub fn get_list_of_processes(&self) -> Option<&Vec<Value>> {
        let process_list = &self.config_json["processes"];
        Some(process_list.as_array().unwrap())
    }
}
