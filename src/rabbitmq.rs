
use deadpool_lapin::{Manager, Pool, PoolError, lapin::{ConnectionProperties, options::{QueueDeclareOptions, BasicConsumeOptions, BasicAckOptions}, types::FieldTable}};
use std::time::Duration;
use tokio_amqp::*;
use async_trait::async_trait;
use crate::models::{RMQResult,Connection};

pub struct RabbitMQClient {
    queue: String,
    addr: String,
    // manager: Manager
}

impl RabbitMQClient {
    fn new(addr: String, queue: String) -> Self {
        Self {
            queue: queue,
            addr: addr,
            // manager: Manager::new(addr, ConnectionProperties::default()),
        }
    }

    async fn consume(&self) {

        let manager = Manager::new(self.addr, ConnectionProperties::default().with_tokio());
        let pool: Pool = deadpool::managed::Pool::builder(manager)
            .max_size(10)
            .build()
            .expect("can create pool");
        
        let rmq_con = Self::get_rmq_con(pool).await.map_err(|e| {
            eprintln!("could not get rmq con: {}", e);
            e
        })?;
    }

    async fn rmq_listen(pool: Pool) -> Result<()> {
        let mut retry_interval = tokio::time::interval(Duration::from_secs(5));
        loop {
            retry_interval.tick().await;
            println!("connecting rmq consumer...");
            match Self::init_rmq_listen(pool.clone()).await {
                Ok(_) => println!("rmq listen returned"),
                Err(e) => eprintln!("rmq listen had an error: {}", e),
            };
        }
    }
    
    async fn init_rmq_listen(pool: Pool) -> Result<()> {
        let rmq_con = Self::get_rmq_con(pool).await.map_err(|e| {
            eprintln!("could not get rmq con: {}", e);
            e
        })?;
        let channel = rmq_con.create_channel().await?;
    
        let queue = channel
            .queue_declare(
                "hello",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;
        println!("Declared queue {:?}", queue);
    
        let mut consumer = channel
            .basic_consume(
                "hello",
                "my_consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;
    
        println!("rmq consumer connected, waiting for messages");
        while let Some(delivery) = consumer.next().await {
            if let Ok((channel, delivery)) = delivery {
                println!("received msg: {:?}", delivery);
                channel
                    .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                    .await?
            }
        }
        Ok(())
    }

    async fn get_rmq_con(pool: Pool) -> RMQResult<Connection> {
        let connection = pool.get().await?;
        Ok(connection)
    }
}