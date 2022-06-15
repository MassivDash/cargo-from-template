use core::panic;
use regex::Regex;
use std::{
    fs,
    io::{Read, Write},
    path::{ Path},
};
#[derive(Debug, Clone)]
pub struct TemplateFile {
    pub name: String,
    pub path: String,
    pub is_folder: bool,
}
#[derive(Debug)]

pub struct Template {
    pub name: String,
    pub has_folder: bool,
    pub files: Vec<TemplateFile>,
}

pub fn get_files(path: &str, folders_only: bool) -> Vec<Template> {
    let mut files: Vec<Template> = Vec::new();
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        println!("{:#?}", path.is_file());
        let filename = path.file_name().unwrap().to_str().unwrap();

        if path.is_dir() && folders_only {
            files.push(Template {
                name: filename.to_string(),
                files: Vec::new(),
                has_folder: false,
            });
        }

        if !folders_only {
            let mut inner_files: Vec<TemplateFile> = Vec::new();
            if path.is_dir() {
                for entry in fs::read_dir(&path).unwrap() {
                    let entry = entry.unwrap();
                    let sub_path = entry.path();
                    let sub_filename = path.file_name().unwrap().to_str().unwrap();
                    inner_files.push(TemplateFile {
                        name: sub_filename.to_string(),
                        path: sub_path.to_string_lossy().to_string(),
                        is_folder: sub_path.is_dir(),
                    });
                }
            }
            if path.is_file() {
                println!("fuck");
                inner_files.push(TemplateFile {
                    name: filename.to_string(),
                    path: path.to_string_lossy().to_string(),
                    is_folder: path.is_dir(),
                });
            }
            files.push(Template {
                name: filename.to_string(),
                files: inner_files,
                has_folder: path.is_dir(),
            });
        }
    }
    return files;
}

pub fn look_for_variables(template_files: &Vec<TemplateFile>) -> Vec<String> {
    let mut variables: Vec<String> = Vec::new();
    let regex = Regex::new(r"%\d%").unwrap();

    for file in template_files {
        let contents = open_and_read(&file.path);
        for vars in regex.find_iter(&contents) {
            let var = vars.as_str();
            if !variables.contains(&var.to_string()) {
                variables.push(var.to_string());
            }
        }
    }
    return variables;
}

pub fn output_dir_check(path: &String) {
    let path = Path::new(path);
    if path.is_dir() {
        return;
    }
    let dir = fs::create_dir_all(path);
    if dir.is_err() {
        panic!("Could not create output directory");
    }
    return;
}

pub fn open_and_read(path: &str) -> String {
    let mut file = fs::File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents;
}

pub fn write_to_file(path: &str, contents: &str) {
    let mut file = fs::File::create(path).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}
