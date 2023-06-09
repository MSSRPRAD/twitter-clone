use crate::config::config::Config;

use sqlx::mysql::MySqlPool;
pub mod config;
pub mod handler;

pub struct AppState {
    pub db: MySqlPool,
    pub env: Config,
    // pub sessiondb: Data<deadpool_redis::Pool>,
}
