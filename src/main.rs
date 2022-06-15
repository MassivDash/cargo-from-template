use std::{fs, io::{Read, Write}};
use chrono::prelude::*;

fn main() {

    let date = Local::now().format("%Y-%m-%d").to_string();
    let template = open_and_read("templates/md_post/%1%.md");
    let stage1 = template.replace("%1%", "Hello, world!").replace("%4%", "error");
    let stage2 = stage1.replace("%2%", "This is a test.");
    let stage3 = stage2.replace("__DATE__" , &date);
    println!("{}", stage3);
    write_to_file("test/newTitle.md", &stage3)
}


fn open_and_read (path: &str) -> String {
    let mut file = fs::File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents;
}  

fn write_to_file (path: &str, contents: &str) {
    let mut file = fs::File::create(path).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}



// 1. Read the templates folders and propose on that choices to the user
// Name of the template is the name of the folder
// After choosing the template, read the template contents, find out how many variables are in the template
// and ask the user to fill them in.
// Write down the template in the existing path 