use actix_web::{middleware::Logger, web, App, HttpServer, HttpResponse};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting HTTP server at http://localhost:8484");

    HttpServer::new(|| {
        App::new()
            .route("/callback", web::get().to(callback))
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8484))?
    .run()
    .await
}

async fn callback() -> HttpResponse {
    HttpResponse::Accepted().body("initiated")
}