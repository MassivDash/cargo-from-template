use chrono::prelude::*;


pub fn get_time() -> String {
    let now = Local::now();
    return now.format("%Y-%m-%d").to_string();
}