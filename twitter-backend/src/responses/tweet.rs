use crate::schema::tweet::TweetModel;
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TweetModelResponse {
    pub tweet_id: i32,
    pub user_id: i32,
    pub parent_id: Option<i32>,
    pub content: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub fn make_tweet_model_response(tweet: &TweetModel) -> TweetModelResponse {
    TweetModelResponse {
        tweet_id: tweet.tweet_id.to_owned(),
        user_id: tweet.user_id.to_owned(),
        parent_id: tweet.parent_id.to_owned(),
        content: tweet.content.to_owned(),
        created_at: tweet.created_at.to_owned(),
    }
}