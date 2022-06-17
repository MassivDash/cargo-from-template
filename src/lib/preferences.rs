extern crate preferences;
use preferences::{AppInfo, PreferencesMap, Preferences, PreferencesError, prefs_base_dir};

const APP_INFO: AppInfo = AppInfo{name: "CARGOFROMTEMPLATE", author: "Lukasz Celitan"};

pub fn get_preferences() -> Result<PreferencesMap, PreferencesError> {
    
    let load_result = PreferencesMap::<String>::load(&APP_INFO, "templates_path");
    return load_result;
}

pub fn store_preferences(preferences: &PreferencesMap, templates_path: &str) -> Result<(), PreferencesError> {
    let save_result = preferences.save(&APP_INFO, templates_path);
    return save_result;
}

