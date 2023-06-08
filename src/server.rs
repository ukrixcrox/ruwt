use actix_web::{App, HttpServer};
use actix_files::Files;

#[actix_web::main]
pub async fn start_server(data: Vec<String>) -> std::io::Result<()>{
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    let mut port_data = data[2].clone();

    let ip = data[1].clone();

    let port = port_data.as_mut_ptr() as u16;

    HttpServer::new(move || {
        let dir = data[0].clone(); // this is probably super unsave, but yolo

        App::new()
                .service(Files::new("/", dir).index_file("index.html"))
        
    })
    .bind((ip, port))?
    .run()
    .await
}
