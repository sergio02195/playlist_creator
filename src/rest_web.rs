use actix_files::Files;
use actix_web::{middleware::Logger, web, App, HttpServer, HttpResponse};
use models::SpotifyAuth;

mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting HTTP server at http://localhost:8888");

    HttpServer::new(|| {
        App::new()
            .route("/callback", web::get().to(callback))
            .service(Files::new("/", "./public").index_file("index.html"))
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8888))?
    .run()
    .await

    // GET / => Home
    // GET /Authorize => redirect to spotify
    // GET /agree => agree page
    // GET /public/... => ./public/..
}

async fn callback(code: web::Query<SpotifyAuth>) -> HttpResponse {
    println!("{}", code.code);
    HttpResponse::Accepted().body("initiated")
    // make request to api
    // redirect to agree
}