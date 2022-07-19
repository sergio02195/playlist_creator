use std::{io};

use actix_web::{HttpServer, App};

#[actix_web::main]
async fn main() -> io::Result<()> {

    HttpServer::new(|| {
        App::new()
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
