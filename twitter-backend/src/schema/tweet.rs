use serde_derive::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct TweetModel {
    pub tweet_id: i32,
    pub user_id: i32,
    pub parent_id: Option<i32>,
    pub content: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub likes: i32,
    pub retweets: i32,
    pub quotes: i32,
    pub views: i32,
    pub replies: i32,
    pub quote_id: Option<i32>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct TweetCreateResponse {
    pub content: String
}
