use inquire::{error::InquireError, Select, Text};

pub fn template_choices(templates: Vec<String>) -> String {
    let ans: Result<String, InquireError> = Select::new("Select template:", templates).prompt();

    match ans {
        Ok(choice) => return choice,
        Err(_) => panic!("There was an error, please try again"),
    }
}

#[derive(Debug, Clone)]
pub struct Vars {
    pub name: String,
    pub value: String,
}

pub fn set_vars(var_name: String) -> Vars {
    let value = Text::new(&format!("What is the value for variable {} ? ", var_name)).prompt();

    match value {
        Ok(value) => return Vars {
            name: var_name,
            value: value,
        },
        Err(_) => panic!("An error happened when asking for your vars, try again later."),
    }
}

pub fn provide_template() -> String {
    let ans: Result<String, InquireError> = Text::new("Provide your templates folder path (absolute):").prompt();

    match ans {
        Ok(choice) => return choice,
        Err(_) => panic!("There was an error, please try again"),
    }
}
