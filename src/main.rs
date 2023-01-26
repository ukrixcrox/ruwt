// Simple cli tool to create web boilerplate code 
// Could use fs::create_dir function

use clap::Parser;
use std::fs;

///Cli tool to create web boilerplate code
#[derive(Parser)]
#[command(author, version, about)]
struct Args{
    ///Project name
    project_name:String,
}


fn main() {
    let args = Args::parse();

    // create root directory 
    create_root_dir(&args.project_name);
}

fn create_root_dir(name: &String) {
    fs::DirBuilder::new()
        .create(name)
        .unwrap_or_else(|e| println!("Error: {}",e));

    //create the html boilerplate code
    create_html_boil(name);

    // create "static" directory
    let path_static = name.to_owned() + &String::from("/static");
    create_static_dir(&path_static);

    //create css boilerplate code
    create_css_boil(&path_static);

    //create JavaScript boilerplate code
    create_js_boil(path_static);
}

fn create_static_dir(path: &String) {
    fs::DirBuilder::new()
        .create(path)
        .unwrap_or_else(|e| println!("Error: {}", e));
}

fn create_html_boil(root_path: &String){
    let file_path = root_path.to_owned() + &String::from("/index.html");
    let html_boil = "<!DOCTYPE html>
<html lang='en'>
<head>
  <title>Html-Boil</title>
  <meta charset='UTF-8' />
  <link rel='stylesheet' type='text/css' href='static/style.css'/>
  <script type='text/javascript' src='static/index.js'></script>
</head>
<body>
  <h1>Hello, world!</h1>
</body>
</html>";

    fs::write(file_path, html_boil)
        .unwrap_or_else(|e| println!("Error: {}", e));
}

fn create_css_boil(path_static: &String){
    let file_path = path_static.to_owned() + &String::from("/style.css");

    let css_boil = "html{
        background-color: black;
    }

    h1{
        color:white;
        text-align: center;
    }";

    fs::write(file_path, css_boil)
        .unwrap_or_else(|e| println!("Error: {}", e));
}

fn create_js_boil(path_static: String){
    let file_path = path_static + &String::from("/index.js");
    let js_boil = "console.log('Hello, World!')";

    fs::write(file_path, js_boil)
        .unwrap_or_else(|e| println!("Error: {}", e));
}


