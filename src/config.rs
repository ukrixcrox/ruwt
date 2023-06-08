use std::fs;
use toml::Table;
use dirs::config_dir;

const CONFIG_DATA: &str = "html = '''
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
'''

css = '''
html{
        background-color: black;
    }

    h1{
        color:white;
        text-align: center;
    }
'''

js = '''
console.log('Hello, World!');
'''";

/// creates config.toml at ~/.config/ruwt_config/config.toml
pub fn create_config() {

    let config_dir_string = config_dir().unwrap().to_str().unwrap().to_owned(); 

    fs::DirBuilder::new().create(config_dir_string + "/ruwt_config/").unwrap();

    let file_path =  config_dir().unwrap().to_str().unwrap().to_owned() + "/ruwt_config/config.toml";
    fs::write(file_path, CONFIG_DATA).unwrap();
}

/// parses the ruwt_config/config.toml
pub fn parse_config() -> Vec<String>{

    let mut parsed_data= vec![];
    
    let toml_data = fs::read_to_string(dirs::config_dir().unwrap().to_string_lossy().to_string() + "/ruwt_config/config.toml")
                            .expect("Failed to read the TOML file");


    let value = toml_data.parse::<Table>().unwrap(); 
    let valid_values = vec!["html", "css", "js"];

    for valid_value in valid_values{
        let parsing_value = value[valid_value].as_str().unwrap();
        parsed_data.push(parsing_value.to_owned());
    }

    parsed_data

}

const SERVERCONFIG_DATA: &str = "projectfolder_path =  '''path/to/ruwt/project/directory'''
\nIp_address = '''127.0.0.1'''
\nPort= '''8080'''";

/// creates serverconfig.toml in the generated root dir
pub fn create_serverconfig(project_dir: String){
    let file_path = project_dir.to_owned() + "/serverconfig.toml";

    fs::write(file_path,SERVERCONFIG_DATA).unwrap();
    
}

/// parses serverconfig.toml
pub fn parse_serverconfig(project_dir: String) -> Vec<String>{
    let mut parsed_data = vec![];
    
    let toml_data = fs::read_to_string(project_dir.to_owned() + "/serverconfig.toml")
                            .expect("Failed to read TOML file");

    let value = toml_data.parse::<Table>().unwrap();
    let valid_values = vec!["projectfolder_path", "Ip_address", "Port"];

    for valid_value in valid_values{
        let parsing_value = value[valid_value].as_str().unwrap();
        parsed_data.push(parsing_value.to_owned());
    }

    parsed_data
}

