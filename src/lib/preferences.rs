extern crate confy;
use crate::display::questions;
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfyConfig {
    pub templates_path: String,
}

impl Default for ConfyConfig {
    fn default() -> Self {
        ConfyConfig {
            templates_path: "".to_string(),
        }
    }
}

pub fn get_templates_path() -> String{
    let cfg: Result<ConfyConfig, confy::ConfyError> = confy::load("cargo-from_template");
    match cfg {
        Ok(cfg) => {
            if cfg.templates_path == "" {
                println!("No templates path setup");
                let templates_path = questions::provide_template();
                store_preferences(&templates_path).unwrap();
                return templates_path;

            }
            return cfg.templates_path;
        }
        Err(err) => {
            panic!("{}", err);
        }
    }
}

pub fn store_preferences(templates_path: &str) -> Result<(), confy::ConfyError> {
    let cfg = ConfyConfig {
        templates_path: templates_path.to_string(),
    };
    let save_result = confy::store("cargo-from_template", cfg);
    return save_result
}

