use crate::display::{questions, screen::spacer};
use confy::{load, store, ConfyError};
use serde_derive::{Deserialize, Serialize};

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

pub fn get_templates_path() -> String {
    let cfg: Result<ConfyConfig, confy::ConfyError> = load("cargo-from_template", None);
    match cfg {
        Ok(cfg) => {
            if cfg.templates_path == "" {
                spacer();
                println!("No templates path setup");
                println!("Check how to make templates at https://github.com/MassivDash/cargo-from-template");
                spacer();
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

pub fn store_preferences(templates_path: &str) -> Result<(), ConfyError> {
    let cfg = ConfyConfig {
        templates_path: templates_path.to_string(),
    };
    let save_result = store("cargo-from_template", None, cfg);
    return save_result;
}
