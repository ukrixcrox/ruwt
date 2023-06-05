use std::fs;
use toml::{Table, Value};
use dirs;
use std::path::{PathBuf, Path};

/// creates html boilerplate code in path/index.html
pub fn get_boil(root_path: &String){
    let file_path = root_path.to_owned() + &String::from("/index.html");
    
    let toml_data = fs::read_to_string(dirs::config_dir().unwrap().to_string_lossy().to_string() + &String::from("/ruwt_config/config.toml"))
                            .expect("Failed to read the TOML file");

    /* 
    let parsed_data: Value = toml::from_str(&toml_data)
                            .expect("Failed to parse the TOML file");
    */

    let value = toml_data.parse::<Table>().unwrap(); 
    

 /*    fs::write(file_path, html_boil)
        .unwrap_or_else(|e| println!("Error: {}", e));
    */
}

/*
/// creates css boilerplate code in path/style.css
pub fn create_css_boil(path_static: &String){
    let file_path = path_static.to_owned() + &String::from("/style.css");

    fs::write(file_path, css_boil)
        .unwrap_or_else(|e| println!("Error: {}", e));
}

/// creates js boilerplate code in path/index.js
pub fn create_js_boil(path_static: String){
    let file_path = path_static + &String::from("/index.js");

    fs::write(file_path, js_boil)
        .unwrap_or_else(|e| println!("Error: {}", e));
}
*/