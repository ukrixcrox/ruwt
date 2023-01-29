// Simple cli tool to create web boilerplate code 
// Could use fs::create_dir function

use clap::Parser;
use std::fs;
use std::path;

///Cli tool to create web boilerplate code
#[derive(Parser)]
#[command(author="krixcrox<falkwitte@github>", version, about)]
struct Opts{
    ///Project name
    project_name:String,
    
    // creates a dir ~/.ruwt_rocket_webserver and copys this webserver into the project folder
    // every time this flag is set
    // changing the webserver in ~/.ruwt_rocket_webserver can change the webserver globaly 
    // currently does Nothing!
    ///Create rocket webserver (currently not working)
    #[arg(short='w', long)]
    webserver:bool,

    // this gets a file path, opens the file and copys all the bytes into a 
    // new file with the same name in the project folder.
    ///add file with Path (currently not working)
    #[arg(short='f', long)]
    add_file:String,

    ///Verbose output
    #[arg(short='v', long)]
    verbose:bool,

    ///create a go webserver dir structure instead of a vanilla website 
    #[arg(short='g', long)]
    go_dir_struc:bool,
}


fn main() {
    let args = Opts::parse();

    if args.webserver{
        println!("Webserver");
    }

    if args.verbose{
        output_verbose(&args.project_name, args.go_dir_struc)
    }

    if args.add_file{
        println!("add file with path")
    }

    if !args.go_dir_struc{
        // create root directory 
        create_root_dir(&args.project_name);
    } else if args.go_dir_struc{
        create_go_dir_struc(&args.project_name)
    }

}

fn add_file(file_path:&String, project_name: &String) -> String{
    let file_name = path::Path::new(file_path)
        .file_name()
        .unwrap()
        .to_string()
        .unwrap();
    
    let project_file_path = project_name.to_owned() + file_name;

    fs::copy(file_path, project_file_path);

    return file_name.to_string();
}


fn output_verbose(root_dir: &String, go_dir_struc:bool, add_file:bool, file_name:String) {

    if !go_dir_struc{
        println!("Creating Directory structure:

    {}/
    |- index.html
    |- static/
        |- style.css
        |- index.js
    ", root_dir)
    } else if go_dir_struc{
        println!("Creating Directory structure:
        
        {}/
    |- cmd/
        |-{}/
    |- handler/
        |-frontend/
        |-backend/
    |- modules/
    |- server/
    |- static
        |-css/
    |- templates/
    ", root_dir, root_dir)
    }else if add_file && !go_dir_struc{
        println!("Creating Directory structure:

    {}/
    |- {}
    |- index.html
    |- static/
        |- style.css
        |- index.js
    ", root_dir, file_name);
    }
}

fn create_go_dir_struc(project_name:&String){ 

    let cmd_path = String::from("/cmd/") + project_name;
    let path_arr = [cmd_path, String::from("/handler/frontend"), String::from("/handler/backend"), String::from("/modules"), String::from("/server"), String::from("/static/css"), String::from("/templates")];
    
    for items in path_arr{
        fs::create_dir_all(project_name.to_owned()+&items).unwrap_or_else(|e| println!("Error: {}", e));
    }

}

fn create_root_dir(project_name: &String) {
    fs::DirBuilder::new()
        .create(project_name)
        .unwrap_or_else(|e| println!("Error: {}",e));

    //create the html boilerplate code
    create_html_boil(project_name);

    // create "static" directory
    let path_static = project_name.to_owned() + &String::from("/static");
    create_static_dir(&path_static);

    //create css boilerplate code
    create_css_boil(&path_static);

    //create JavaScript boilerplate code
    create_js_boil(path_static);
}

fn create_static_dir(path_static: &String) {
    fs::DirBuilder::new()
        .create(path_static)
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


