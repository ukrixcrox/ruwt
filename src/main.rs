// Simple cli tool to create web boilerplate code 
// Could use fs::create_dir function
//
//
// TODO:
// - write the html and static boil code into a directory ~/.ruwt_config 
// so those files can be globaly easily configured

mod commands;
mod boil;
mod server;
mod config;

use clap::Parser;
use crate::server::start_server;
use crate::commands::{add_file, output_verbose, create_project};
use crate::config::{parse_config};
use std::path::Path;

///Cli tool to create web boilerplate code
#[derive(Parser)]
#[command(author="krixcrox<falkwitte@github>", version, about)]
struct Opts{
    ///Project name
    project_name:String,

    ///Serve files with actix webserver
    #[arg(short='r')]
    #[arg(long="run")]
    start_server:bool,
    
    // this gets a file path, opens the file and copys all the bytes into a 
    // new file with the same name in the project folder.
    ///add a arbitrary file to the project folder
    #[arg(short='f')]
    #[arg(long="file-path")]
    file_path:Option<String>,

    ///Verbose output
    #[arg(short='v')]
    #[arg(long="verbose")]
    verbose:bool,
}

fn main() {
    let args = Opts::parse();



    if args.verbose{
        output_verbose(&args.project_name);
    }

    let config_dir_string = dirs::config_dir().unwrap().to_string_lossy().to_string();
    let config_file_path = config_dir_string + &String::from("/ruwt_config/config.toml");

    if Path::new(&config_file_path).exists() {
        create_project(&args.project_name, parse_config());
    }else{
        println!("Error: No config file");
        println!("Generated config file at ~/.config/ruwt_config/config.toml");
        config::create_config();
        std::process::exit(0x100);
    }
    
    // checks whether the file_path field of the struct args has some value 
    // and binds this value to the local variable file_path
    if let Some(file_path) = &args.file_path{
        add_file(file_path, &args.project_name); 
    }

    if args.start_server{
        let dir = &args.project_name;
        start_server(dir.to_owned()).unwrap();
    }
}

// TODO: refactor to commands.rs