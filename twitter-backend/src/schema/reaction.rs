use serde_derive::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct ReactionModel {
    pub tweet_id: i32,
    pub username: String,
    pub reaction_id: i32,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct ImplicitRating {
    pub user_id: i32,
    pub tweet_id: i32,
}
