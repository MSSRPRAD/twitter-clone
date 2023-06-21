use async_recursion::async_recursion;
use actix_web::web;

use crate::{responses::tweet::CreateTweetModelResponse, config::AppState, errors::auth::AuthError, schema::tweet::TweetModel};

pub async fn create_tweet(
    body: web::Json<CreateTweetModelResponse>,
    data: web::Data<AppState>,
    username: String,
) -> AuthError {
    // Insert the user into the database
    let _insert_result = sqlx::query_as!(
        TweetModel,
        "INSERT INTO TWEETS (username, parent_id, content, quote_id)
        VALUES 
            (?, ?, ?, ?);",
        username,
        body.parent_id,
        body.content,
        body.quote_id,
    )
    .execute(&data.db)
    .await;
    return AuthError::NoError;
}

pub async fn most_recent_tweet_from_username(username: String, data: &web::Data<AppState>) -> Option<TweetModel> {
    let option_tweet = sqlx::query_as!(
        TweetModel,
        "SELECT 
        tweet_id,
        username,
        parent_id,
        content,
        created_at,
        likes,
        retweets,
        quotes,
        views,
        replies,
        quote_id
    FROM TWEETS
    WHERE
    username = ?
    ORDER BY
    created_at
    ASC;",
        username
    )
    .fetch_all(&data.db)
    .await;

    match option_tweet {
        Ok(_) => {
            return Option::from(option_tweet.unwrap().last().unwrap()).cloned();
        }
        Err(_) => {
            return None;
        }
    }
}

pub async fn all_tweets_quoting_tweetid(tweet_id: i32, data: &web::Data<AppState>) -> Option<Vec<TweetModel>> {
    let option_tweets: Result<Vec<TweetModel>, sqlx::Error> = sqlx::query_as!(
        TweetModel,
        "SELECT 
        tweet_id,
        username,
        parent_id,
        content,
        created_at,
        likes,
        retweets,
        quotes,
        views,
        replies,
        quote_id
    FROM TWEETS
    WHERE
    quote_id = ?
    ORDER BY
    created_at
    ASC;",
        tweet_id
    )
    .fetch_all(&data.db)
    .await;

    match option_tweets {
        Ok(_) => {
            return Option::from(option_tweets.unwrap());
        }
        Err(_) => {
            return None;
        }
    }
}

pub async fn all_tweets_replying2_tweetid(tweet_id: i32, data: &web::Data<AppState>) -> Option<Vec<TweetModel>> {
    let option_tweets: Result<Vec<TweetModel>, sqlx::Error> = sqlx::query_as!(
        TweetModel,
        "SELECT 
        tweet_id,
        username,
        parent_id,
        content,
        created_at,
        likes,
        retweets,
        quotes,
        views,
        replies,
        quote_id
    FROM TWEETS
    WHERE
    parent_id = ?
    ORDER BY
    created_at
    ASC;",
        tweet_id
    )
    .fetch_all(&data.db)
    .await;

    match option_tweets {
        Ok(_) => {
            return Option::from(option_tweets.unwrap());
        }
        Err(_) => {
            return None;
        }
    }
}

#[async_recursion]
pub async fn parent_tweet_chain_from_tweetid(
    tweet_id: i32,
    tweet_chain: Option<Vec<TweetModel>>,
    data: &web::Data<AppState>,
) -> Result<Option<Vec<TweetModel>>, sqlx::Error> {
    let option_tweet = sqlx::query_as!(
        TweetModel,
        "SELECT
            tweet_id,
            username,
            parent_id,
            content,
            created_at,
            likes,
            retweets,
            quotes,
            views,
            replies,
            quote_id
        FROM TWEETS
        WHERE
            tweet_id = ?",
        tweet_id,
    )
    .fetch_optional(&data.db)
    .await?;

    match option_tweet {
        Some(tweet) => {
            let mut tweets = match tweet_chain {
                Some(mut chain) => {
                    chain.push(tweet.clone());
                    chain
                }
                None => vec![tweet.clone()],
            };
            if let Some(parent_id) = tweet.parent_id {
                return parent_tweet_chain_from_tweetid(parent_id, Some(tweets), data).await;
            } else {
                return Ok(Some(tweets));
            }
        }
        None => {
            return Ok(tweet_chain);
        }
    }
}

pub async fn tweet_quoted(quote_id: i32, data: &web::Data<AppState>) -> Option<TweetModel> {
    let option_tweet: Result<TweetModel, sqlx::Error> = sqlx::query_as!(
        TweetModel,
        "SELECT 
        tweet_id,
        username,
        parent_id,
        content,
        created_at,
        likes,
        retweets,
        quotes,
        views,
        replies,
        quote_id
    FROM TWEETS
    WHERE
    quote_id = ?
    ORDER BY
    created_at
    ASC;",
        quote_id
    )
    .fetch_one(&data.db)
    .await;

    match option_tweet {
        Ok(_) => {
            return Option::from(option_tweet.unwrap());
        }
        Err(_) => {
            return None;
        }
    }
}