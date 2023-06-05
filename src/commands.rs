use std::path;
use std::fs;

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