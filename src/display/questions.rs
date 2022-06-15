use inquire::{error::InquireError, Select};


pub fn template_choices (templates: Vec<String>) -> String {
    let ans: Result<String, InquireError> = Select::new("Select template:", templates).prompt();

match ans {
    Ok(choice) => return choice,
    Err(_) => panic!("There was an error, please try again"),
  }
}