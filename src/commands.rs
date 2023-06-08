use std::path;
use std::fs;
use crate::boil;
use crate::config;

pub fn add_file(file_path:&String, project_name: &String){
    let file_name = path::Path::new(file_path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    let project_file_path = project_name.to_owned() + "/" + file_name;
   
    // creates empty file to copy bytes to
    fs::write(&project_file_path, "")
        .unwrap_or_else(|e| println!("Error: {}", e));

    fs::copy(file_path, project_file_path).unwrap();
}

pub fn output_verbose(root_dir: &String) {
        println!("Creating Directory structure:

    {}/
    |- index.html
    |- static/
        |- style.css
        |- index.js
    ", root_dir)
    }

pub fn create_project(project_name: &String, data: Vec<String>) {
    fs::DirBuilder::new()
        .create(project_name)
        .unwrap_or_else(|e| println!("Error: {}", e));

    boil::create_html_file(project_name, &data[0]);

    config::create_serverconfig(project_name.to_owned());

    let path_static = project_name.to_owned() + &String::from("/static");
    create_static_dir(&path_static);

    boil::create_css_file(&path_static, &data[1]);

    boil::create_js_file(&path_static, &data[2]);
}

/// helper function for create_project
fn create_static_dir(path_static: &String) {
    fs::DirBuilder::new()
        .create(path_static)
        .unwrap_or_else(|e| println!("Error: {}", e));
}




