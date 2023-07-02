use crate::schema::tweet::TweetModel;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TweetModelResponse {
    pub username: String,
    pub tweet_id: i32,
    pub parent_id: Option<i32>,
    pub content: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub reactions: i32,
    pub retweets: i32,
    pub quotes: i32,
    pub views: i32,
    pub replies: i32,
    pub quote_id: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTweetModelResponse {
    pub parent_id: Option<i32>,
    pub content: String,
    pub quote_id: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineTweets {
    pub tweet_id: i32,
    pub rating: i32,
}


pub fn make_tweet_model_response(tweet: &TweetModel) -> TweetModelResponse {
    TweetModelResponse {
        username: tweet.username.to_owned(),
        tweet_id: tweet.tweet_id.to_owned(),
        parent_id: tweet.parent_id.to_owned(),
        content: tweet.content.to_owned(),
        created_at: tweet.created_at.to_owned(),
        reactions: tweet.reactions.to_owned(),
        retweets: tweet.retweets.to_owned(),
        quotes: tweet.quotes.to_owned(),
        views: tweet.views.to_owned(),
        replies: tweet.replies.to_owned(),
        quote_id: tweet.quote_id.to_owned(),
    }
}
