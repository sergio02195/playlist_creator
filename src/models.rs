use deadpool_lapin::{PoolError, lapin};
use serde::{Serialize, Deserialize};
use std::result::Result as StdResult;
use thiserror::Error as ThisError;
use warp::{Rejection};
use chrono::{DateTime, Utc};

#[derive(Serialize,Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
}

#[derive(Serialize,Deserialize)]
pub struct StartData {
    pub origin: DateTime<Utc>,
    pub token: String,
}

pub type WebResult<T> = StdResult<T, Rejection>;
pub type RMQResult<T> = StdResult<T, PoolError>;
pub type Result<T> = StdResult<T, Error>;

pub type Connection = deadpool::managed::Object<deadpool_lapin::Manager>;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("rmq error: {0}")]
    RMQError(#[from] lapin::Error),
    #[error("rmq pool error: {0}")]
    RMQPoolError(#[from] PoolError),
}

impl warp::reject::Reject for Error {}