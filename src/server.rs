use crate::config::ServerConfigStruct;
use actix_files::Files;
use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer, Responder};

pub async fn start_server(data: ServerConfigStruct) -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let ip = data.ip_address.clone();

    let port = data.port;

    log::info!("Running on: {ip}:{port}");

    HttpServer::new(move || {
        let dir = data.projectfolder_path.clone(); // this is probably super unsave, but yolo

        App::new()
            .wrap(Logger::default())
            .service(Files::new("/", dir).index_file("index.html"))
    })
    .bind((ip, port))?
    .run()
    .await
}
