use futures::join;
use http_health_check::HTTPHealthCheck;
use rabbitmq::RabbitMQClient;
use use_case::start_user;

mod http_health_check;
mod models;
mod rabbitmq;
mod use_case;

extern crate json;

#[tokio::main]
async fn main() {
    println!("Started server at localhost:8000");
    let health_check = HTTPHealthCheck::new();
    let rabbitmq_consumer = RabbitMQClient::new(
        "amqp://guest:guest@localhost:5672//".to_string(),
        "".to_string(),
        start_user,
    );
    let _ = join!(health_check.start(), rabbitmq_consumer.consume());
}
