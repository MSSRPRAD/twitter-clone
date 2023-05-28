use sqlx::mysql::{MySqlPool};
use crate::config::config::Config;
pub mod config;
pub mod handler;

pub struct AppState {
    pub db: MySqlPool,
    pub env: Config,
}