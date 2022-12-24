use crate::models::{Connection, RMQResult, Result};
use deadpool_lapin::{
    lapin::{
        options::{BasicAckOptions, BasicConsumeOptions, QueueDeclareOptions},
        types::FieldTable,
        ConnectionProperties,
    },
    Manager, Pool,
};
use futures::StreamExt;
use std::time::Duration;
use tokio_amqp::*;

pub struct RabbitMQClient {
    queue: String,
    addr: String,
    // manager: Manager
}

impl RabbitMQClient {
    pub fn new(addr: String, queue: String) -> Self {
        Self {
            queue: queue,
            addr: addr,
            // manager: Manager::new(addr, ConnectionProperties::default()),
        }
    }

    pub async fn consume(&self) -> Result<()> {
        let manager = Manager::new(&self.addr, ConnectionProperties::default().with_tokio());
        let pool: Pool = deadpool::managed::Pool::builder(manager)
            .max_size(10)
            .build()
            .expect("can create pool");

        self.rmq_listen(pool).await
    }

    async fn rmq_listen(&self, pool: Pool) -> Result<()> {
        let mut retry_interval = tokio::time::interval(Duration::from_secs(5));
        loop {
            retry_interval.tick().await;
            println!("connecting rmq consumer...");
            match self.init_rmq_listen(pool.clone()).await {
                Ok(_) => println!("rmq listen returned"),
                Err(e) => eprintln!("rmq listen had an error: {}", e),
            };
        }
    }

    async fn get_rmq_con(&self, pool: Pool) -> RMQResult<Connection> {
        let connection = pool.get().await?;
        Ok(connection)
    }

    async fn init_rmq_listen(&self, pool: Pool) -> Result<()> {
        let rmq_con = self.get_rmq_con(pool).await.map_err(|e| {
            eprintln!("could not get rmq con: {}", e);
            e
        })?;
        let channel = rmq_con.create_channel().await?;

        let queue = channel
            .queue_declare(
                &self.queue,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;
        println!("Declared queue {:?}", queue);

        let mut consumer = channel
            .basic_consume(
                &self.queue,
                "my_consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        println!("rmq consumer connected, waiting for messages");
        while let Some(delivery) = consumer.next().await {
            if let Ok((channel, delivery)) = delivery {
                println!("received msg: {:?}", delivery);
                let s = match std::str::from_utf8(&delivery.data) {
                    Ok(v) => v,
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };
            
                println!("result: {}", s);
                channel
                    .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                    .await?
            }
        }
        Ok(())
    }
}
