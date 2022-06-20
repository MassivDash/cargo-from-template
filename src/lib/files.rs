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
    pub name: String, // Name of the template = folder name
    pub has_folder: bool, // Does the template have a folder?
    pub files: Vec<TemplateFile>, // Files in the template to use
}

pub fn get_files(path: &str, folders_only: bool) -> Vec<Template> {
    let mut files: Vec<Template> = Vec::new();
    for entry in fs::read_dir(path).unwrap() {
        let path = entry.as_ref().unwrap().path();
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
                    let file = entry.unwrap();
                    let sub_filename =  file.file_name().to_str().unwrap().to_string();
                    inner_files.push(TemplateFile {
                        name: sub_filename.to_string(),
                        path: file.path().to_str().unwrap().to_string(),
                        is_folder: file.path().is_dir(),
                    });
                }
            }
            if path.is_file() {
                inner_files.push(TemplateFile {
                    name:  path.file_name().unwrap().to_str().unwrap().to_string(),
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
    let regex = Regex::new(r"%\w+%").unwrap();

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


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_files() {
        let files = get_files("templates", false);
        assert_eq!(files.len(), 6);
        assert_eq!(files[0].name, "expressApi");
        assert_eq!(files[0].has_folder, true);
    }
    #[test]
    // Single file test, single vars test
    fn test_look_for_variables() {
        let files = get_files("templates/expressApi", false);
        let variables = look_for_variables(&files[0].files);
        assert_eq!(variables.len(), 1);
        assert_eq!(variables[0], "%name%");
    }

    #[test]
    // Single file test, multiple vars test
    fn test_look_for_multi_variables() {
        let files = get_files("templates/readme", false);
        let variables = look_for_variables(&files[0].files);
        assert_eq!(variables.len(), 5);
        assert_eq!(variables.contains(&"%name%".to_string()), true);
        assert_eq!(variables.contains(&"%description%".to_string()), true);
        assert_eq!(variables.contains(&"%version%".to_string()), true);
        assert_eq!(variables.contains(&"%github_link%".to_string()), true);
    }

    #[test]
    // Subfolder test, multiple vars test
    fn test_subfolder_look_for_multi_variables() {
        let files = get_files("templates/md", false);
        let variables = look_for_variables(&files[0].files);
        assert_eq!(variables.len(), 4);
        assert_eq!(variables.contains(&"%name%".to_string()), true);
        assert_eq!(variables.contains(&"%author%".to_string()), true);
        assert_eq!(variables.contains(&"%excerpt%".to_string()), true);
        assert_eq!(variables.contains(&"%category%".to_string()), true)

    }

}
