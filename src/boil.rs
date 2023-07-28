use std::fs;

/// writes html_data to the html.index file
pub fn create_html_file(project_dir: &String, html_data: &str) {
    let file_path = project_dir.to_owned() + "/index.html";

    fs::write(file_path, html_data).unwrap();
}

/// writes css_data to the /static/style.css
pub fn create_css_file(static_dir: &String, css_data: &str) {
    let file_path = static_dir.to_owned() + "/style.css";

    fs::write(file_path, css_data).unwrap();
}

/// writes js-data to the /static/index.js
pub fn create_js_file(static_dir: &String, js_data: &str) {
    let file_path = static_dir.to_owned() + "/index.js";

    fs::write(file_path, js_data).unwrap();
}
