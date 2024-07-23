// Simple cli tool to create web boilerplate code
// Could use fs::create_dir function
//
//
// TODO:
// - write the html and static boil code into a directory ~/.ruwt_config
// so those files can be globaly easily configured
// INFO: for that i could create a lib that makes writing stuff into the
//       dir easier and more streamlined

mod boil;
mod commands;
mod config;
mod server;

use crate::commands::{add_file, create_project, output_verbose};
use crate::config::parse_config;
use clap::{Parser, Subcommand};
use config::parse_serverconfig;
use std::path::Path;

#[derive(Subcommand)]
pub enum Command {
    /// create a new ruwt project
    Create {
        ///Project name
        project_name: String,

        ///Verbose output
        #[arg(short = 'v')]
        #[arg(long = "verbose")]
        verbose: bool,

        // this gets a file path, opens the file and copys all the bytes into a
        // new file with the same name in the project folder.
        ///add a arbitrary file to the project folder
        #[arg(short = 'f')]
        #[arg(long = "file-path")]
        file_path: Option<String>,
    },
}

///Cli tool to create web boilerplate code
#[derive(Parser)]
#[command(author = "krixcrox<ukrixcrox@github.com>", version, about)]
struct Opts {
    #[clap(subcommand)]
    command: Option<Command>,

    /// start the webserver
    #[arg(long, short = 'r')]
    startserver: bool,
    /*
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
    */
}

#[actix_web::main]
async fn main() {
    let args = Opts::parse();

    if args.startserver {
        server::start_server(parse_serverconfig()).await.unwrap();
    }

    match args.command.unwrap() {
        Command::Create {
            project_name,
            verbose,
            file_path,
        } => {
            let config_dir_string = dirs::config_dir().unwrap().to_string_lossy().to_string();
            let config_file_path = config_dir_string + &String::from("/ruwt_config/config.toml");

            if Path::new(&config_file_path).exists() {
                create_project(&project_name, parse_config());
            } else {
                println!("Error: No config file");
                println!("Generated config file at ~/.config/ruwt_config/config.toml");
                config::create_config();
                std::process::exit(0x100);
            }

            if verbose {
                output_verbose(&project_name);
            }

            // checks whether the file_path field of the struct args has some value
            // and binds this value to the local variable file_path
            if let Some(file_path) = &file_path {
                add_file(file_path, &project_name);
            }
        }
    }
}
