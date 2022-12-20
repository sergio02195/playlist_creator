use futures::join;
use http_health_check::HTTPHealthCheck;

mod http_health_check;
mod models;

#[tokio::main]
async fn main() {
    println!("Started server at localhost:8000");
    let health_check = HTTPHealthCheck::new();
    let _ = join!(
        health_check.start()
    );
}