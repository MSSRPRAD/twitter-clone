use sqlx::mysql::{MySqlPool};
pub mod handler;

pub struct AppState {
    pub db: MySqlPool,
}