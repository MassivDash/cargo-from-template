mod lib;
mod display;

use crate::lib::files;
use crate::display::questions;

// 1. Read the templates folders and propose on that choices to the user
// Name of the template is the name of the folder
// After choosing the template, read the template contents, find out how many variables are in the template
// and ask the user to fill them in.
// Write down the template in the existing path 


fn main() {
    let templates = files::get_files("templates", true);
    let templates_names = templates.iter().map(|x| x.name.clone()).collect::<Vec<String>>();
    let selected_template = questions::template_choices(templates_names);
    let template_files = files::get_files(&format!("templates/{}", selected_template), false);
    
    // Find how many variables are in the template
    let variables: Vec<String> = files::look_for_variables(&template_files[0].files);
    let mut final_vars: Vec<questions::Vars> = Vec::new();
    for var in variables {
        let var_value = questions::set_vars(var);
        final_vars.push(var_value);
    }

    println!("{:#?}", final_vars);
}





