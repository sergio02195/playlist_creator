use crate::models::WebResult;
use warp::{Reply, Filter};

pub struct HTTPHealthCheck {
    port: u16
}

impl HTTPHealthCheck {
    pub fn new() -> Self {
        Self {
            port: 8000
        }
    }

    pub async fn start(&self) {
        let health_route = warp::path!("health")
            .and_then(Self::health_handler);
        warp::serve(health_route)
            .run(([0, 0, 0, 0], self.port)).await
    }

    pub async fn health_handler() -> WebResult<impl Reply> {
        print!("Health check ok");
        Ok("OK")
    }
}
