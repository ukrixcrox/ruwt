// Simple cli tool to create web boilerplate code 
// Could use fs::create_dir function
//
//
// TODO:
// - write the html and static boil code into a directory ~/.ruwt_config 
// so those files can be globaly easily configured

mod boil;

use clap::Parser;
use std::fs;
use std::path;
// for getting the home directory
// use dirs;

///Cli tool to create web boilerplate code
#[derive(Parser)]
#[command(author="krixcrox<falkwitte@github>", version, about)]
struct Opts{
    ///Project name
    project_name:String,
    
    // this gets a file path, opens the file and copys all the bytes into a 
    // new file with the same name in the project folder.
    ///add a arbitrary file to the project folder
    #[arg(short='f', long,)]
    file_path:Option<String>,

    ///Verbose output
    #[arg(short='v', long)]
    verbose:bool,

    ///create a go webserver dir structure instead of a vanilla website dir structure
    #[arg(short='g', long)]
    go_dir_struc:bool,
}

fn main() {
    let args = Opts::parse();

    if args.verbose{
        output_verbose(&args.project_name, args.go_dir_struc)
    }

    if !args.go_dir_struc{
        create_root_dir(&args.project_name);
    } else if args.go_dir_struc{
        create_go_dir_struc(&args.project_name)
    }
    
    // checks whether the file_path field of the struct args has some value 
    // and binds this value to the local variable file_path
    if let Some(file_path) = &args.file_path{
        add_file(file_path, &args.project_name); 
    }
}

fn add_file(file_path:&String, project_name: &String){
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


fn output_verbose(root_dir: &String, go_dir_struc:bool) {

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
    }
}

fn create_go_dir_struc(project_name:&String){ 

    let cmd_path = String::from("/cmd/") + project_name;
    let path_arr = [cmd_path, String::from("/handler/frontend"), String::from("/handler/backend"), String::from("/modules"), String::from("/server"), String::from("/static/css"), String::from("/templates")];
    
    for items in path_arr{
        fs::create_dir_all(project_name.to_owned()+&items).unwrap_or_else(|e| println!("Error: {}",e));
    }

}

fn create_root_dir(project_name: &String) {
    fs::DirBuilder::new()
        .create(project_name)
        .unwrap_or_else(|e| println!("Error: {}",e));

    boil::create_html_boil(project_name);

    let path_static = project_name.to_owned() + &String::from("/static");
    create_static_dir(&path_static);

    boil::create_css_boil(&path_static);

    boil::create_js_boil(path_static);
}

fn create_static_dir(path_static: &String) {
    fs::DirBuilder::new()
        .create(path_static)
        .unwrap_or_else(|e| println!("Error: {}", e));
}




