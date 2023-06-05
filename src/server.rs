use actix_web::{App, HttpServer};
use actix_files::Files;

#[actix_web::main]
pub async fn start_server(dir: String) -> std::io::Result<()>{
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        let dir = dir.clone(); // this is probably super unsave, but yolo

        App::new()
                .service(Files::new("/", dir).index_file("index.html"))
        
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
