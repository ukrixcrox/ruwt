use std::fs;

/// creates html boilerplate code in path/index.html
pub fn create_html_boil(root_path: &String){
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

/// creates css boilerplate code in path/style.css
pub fn create_css_boil(path_static: &String){
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

/// creates js boilerplate code in path/index.js
pub fn create_js_boil(path_static: String){
    let file_path = path_static + &String::from("/index.js");
    let js_boil = "console.log('Hello, World!')";

    fs::write(file_path, js_boil)
        .unwrap_or_else(|e| println!("Error: {}", e));
}