
mod display;
mod lib;

use std::collections::HashMap;

use crate::display::questions;
use crate::display::screen::{hr, spacer, step};
use crate::lib::{files, get_time::get_time, preferences};

// Name of the template is the name of the folder
// After choosing the template, read the template contents, find out how many variables are in the template
// and ask the user to fill them in.
// Write down the template in the existing path




fn main() {
    // Present your self to the user
    spacer();
    step("cargo-from-template: version 0.1.0");
    hr();

    let mut app_settings: HashMap<String, String> = HashMap::new();
    let mut templates_path = "".to_string();

    // 0. Check for the user settings, if not present ask to establish them
    let preferences_settings = preferences::get_preferences();
    match preferences_settings {
        Ok(preferences) => {
            println!("Preferences loaded");
            app_settings = preferences
        }
        Err(err) => {
            println!("Preferences not found");
            println!("{:#?}", err);
            let template = questions::provide_template();
            templates_path = template;
            preferences::store_preferences(&app_settings, &templates_path).unwrap();
        }
    }

    let preferences_settings_1 = preferences::get_preferences().unwrap();
    println!("{:#?}", &preferences_settings_1);

    // 1. Read the templates
    

    let templates = files::get_files(&templates_path, true);
    let templates_names = templates
        .iter()
        .map(|x| x.name.clone())
        .collect::<Vec<String>>();

    // 2. Ask the user to choose a template

    let selected_template = questions::template_choices(templates_names);
    let template = &files::get_files(&format!("{}/{}", templates_path, selected_template), false)[0];
    let template_files = &template.files;

    // 3. Find how many variables are in the template
    let variables: Vec<String> = files::look_for_variables(&template_files);
    let mut final_vars: Vec<questions::Vars> = Vec::new();

    // 4. Iterate through the variables and ask the user to fill them in

    step(&format!(
        "Time to enter your predefined variables, {} in total",
        variables.len()
    ));

    for var in variables {
        let var_value = questions::set_vars(var);
        final_vars.push(var_value);
    }

    // Write the template
    for file in template_files {
        let mut contents = files::open_and_read(&file.path);
        let mut new_path;
        let mut new_folder = format!("{}", &template.name).replace("__DATE__", &get_time());
        if template.has_folder {
            contents = contents.replace("__DATE__", &get_time());
            new_path = format!("{}/{}", &new_folder, file.name);
        } else {
            new_path = format!("{}", file.name);
        }
        for var in &final_vars {
            new_folder = new_folder
                .to_string()
                .replace(&format!("{}", var.name), &var.value);
            contents = contents.replace(&format!("{}", var.name), &var.value);
            new_path = new_path.replace(&format!("{}", var.name), &var.value);
        }

        if template.has_folder {
            files::output_dir_check(&new_folder);
        }

        files::write_to_file(&new_path, &contents);
    }

    step("Success 🚀, files created")
}
