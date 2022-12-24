use futures::join;
use http_health_check::HTTPHealthCheck;
use rabbitmq::RabbitMQClient;

mod http_health_check;
mod models;
mod rabbitmq;

#[tokio::main]
async fn main() {
    println!("Started server at localhost:8000");
    let health_check = HTTPHealthCheck::new();
    let rabbitmc_consumer = RabbitMQClient::new("amqp://guest:guest@localhost:5672//".to_string(), "".to_string() );
    let _ = join!(
        health_check.start(),
        rabbitmc_consumer.consume()
    );
}