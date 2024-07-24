use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::env::current_dir;
use std::fs;
use std::path::PathBuf;

const CONFIG_DATA: &str = "html = \"\"\"
<!DOCTYPE html>
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
</html>
\"\"\"

css = \"\"\"
html{
        background-color: black;
    }

    h1{
        color:white;
        text-align: center;
    }
\"\"\"

js = \"\"\"
console.log('Hello, World!');
\"\"\"";

/// creates config.toml at ~/.config/ruwt_config/config.toml
pub fn create_config() {
    let config_dir_string = config_dir().unwrap().to_str().unwrap().to_owned();

    if !PathBuf::from(config_dir_string.clone() + "/ruwt_config/").exists() {
        fs::DirBuilder::new()
            .create(config_dir_string + "/ruwt_config/")
            .unwrap();
    }

    let file_path = config_dir().unwrap().to_str().unwrap().to_owned() + "/ruwt_config/config.toml";
    fs::write(file_path, CONFIG_DATA).unwrap();
}

#[derive(Deserialize, Serialize)]
pub struct ConfigData {
    pub html: String,
    pub css: String,
    pub js: String,
}

/// parses the ruwt_config/config.toml
pub fn parse_config() -> ConfigData {
    let config_dir_string = dirs::config_dir().unwrap().to_string_lossy().to_string();

    let toml_data = fs::read_to_string(config_dir_string + "/ruwt_config/config.toml")
        .expect("Failed to read the TOML file");

    let value: ConfigData = toml::from_str(&toml_data).unwrap();

    value
}

const SERVERCONFIG_DATA: &str = "projectfolder_path =  \"path/to/ruwt/project/directory\"
\nip_address = \"localhost\"
\nport= 8080";

/// creates serverconfig.toml in the generated root dir
pub fn create_serverconfig(project_dir: String) {
    let file_path = project_dir + "/serverconfig.toml";

    fs::write(file_path, SERVERCONFIG_DATA).unwrap();
}

#[derive(Deserialize, Serialize)]
pub struct ServerConfigStruct {
    pub projectfolder_path: String,
    pub ip_address: String,
    pub port: u16,
}

/// parses serverconfig.toml
pub fn parse_serverconfig() -> ServerConfigStruct {
    let current_dir_string = current_dir().unwrap().to_str().unwrap().to_owned();

    let toml_data = fs::read_to_string(current_dir_string + "/serverconfig.toml")
        .expect("Failed to read TOML file");

    let value: ServerConfigStruct =
        toml::from_str(&toml_data).expect("Error: Failed to parse serverconfig.toml");

    value
}
