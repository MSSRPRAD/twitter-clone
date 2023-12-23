use crate::{
    config::AppState,
    errors::auth::AuthError,
    errors::auth::ErrorResponse,
    responses::tweet::{CreateTweetModelResponse, TimelineTweets},
    schema::reaction::ReactionModel,
    schema::{reaction::ImplicitRating, tweet::{TweetModel, TweetRequestType}, user::UserId},
};
use actix_web::web;
use async_recursion::async_recursion;
use sqlx::Row;

pub async fn reaction_from_username_tweet_id(
    username: String,
    tweet_id: i32,
    data: &web::Data<AppState>,
) -> Option<ReactionModel> {
    let option_reaction = sqlx::query_as!(
        ReactionModel,
        "SELECT reaction_id, tweet_id, username, created_at
        FROM REACTIONS 
        WHERE tweet_id = ? AND username = ?",
        tweet_id,
        username
    )
    .fetch_optional(&data.db)
    .await;
    // println!("{:?}", option_reaction);
    match option_reaction {
        Ok(reaction) => reaction,
        Err(_) => None,
    }
}

pub async fn create_or_remove_reaction(
    username: String,
    tweet_id: i32,
    reaction_id: i32,
    data: web::Data<AppState>,
) -> ErrorResponse {
    if let Some(_tweet) = tweet_from_tweet_id(tweet_id, &data).await {
        if let Some(_user) =
            crate::functions::user::user_from_username(username.clone(), &data).await
        {
            if let Some(_) =
                reaction_from_username_tweet_id(username.clone(), tweet_id, &data).await
            {
                let delete_result = sqlx::query!(
                    "DELETE FROM REACTIONS WHERE tweet_id = ? AND username = ?",
                    tweet_id,
                    username,
                )
                .execute(&data.db)
                .await;
                let _update_result = sqlx::query!(
                    "UPDATE TWEETS
                SET reactions = reactions - 1
                WHERE tweet_id = ?;",
                    tweet_id,
                )
                .execute(&data.db)
                .await;

                if let Ok(_) = delete_result {
                    return ErrorResponse::NoError();
                } else {
                    return ErrorResponse::InternalServerError();
                }
            } else {
                let insertion_result = sqlx::query!(
                    "INSERT INTO REACTIONS (tweet_id, username, reaction_id) VALUES (?, ?, ?)",
                    tweet_id,
                    username,
                    reaction_id,
                )
                .execute(&data.db)
                .await;
                let _update_result = sqlx::query!(
                    "UPDATE TWEETS
                SET reactions = reactions + 1
                WHERE tweet_id = ?;",
                    tweet_id,
                )
                .execute(&data.db)
                .await;

                if let Ok(_) = insertion_result {
                    return ErrorResponse::NoError();
                } else {
                    return ErrorResponse::InternalServerError();
                }
            }
        } else {
            return ErrorResponse::InvalidCredentials();
        }
    } else {
        return ErrorResponse::InvalidTweet();
    }
}

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
    // Increase the quotes of the quoted tweet
    // and the replies of the parent tweet
    match body.quote_id {
        Some(quote_id) => {
            let _update_result = sqlx::query!(
                "UPDATE TWEETS
                SET quotes = quotes + 1
                WHERE tweet_id = ?;",
                quote_id,
            )
            .execute(&data.db)
            .await;
        }
        None => {}
    }
    match body.parent_id {
        Some(parent_id) => {
            let _update_result = sqlx::query!(
                "UPDATE TWEETS
                SET replies = replies + 1
                WHERE tweet_id = ?;",
                parent_id,
            )
            .execute(&data.db)
            .await;
        }
        None => {}
    }
    return AuthError::NoError;
}

pub async fn most_recent_tweet_from_username(
    username: String,
    data: &web::Data<AppState>,
) -> Option<TweetModel> {
    let option_tweet = sqlx::query_as!(
        TweetModel,
        "SELECT 
        tweet_id,
        username,
        parent_id,
        content,
        created_at,
        reactions,
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
            return Option::from(option_tweet.unwrap().last().unwrap().clone());
        }
        Err(_) => {
            return None;
        }
    }
}

pub async fn all_tweets_quoting_tweetid(
    tweet_id: i32,
    data: &web::Data<AppState>,
) -> Option<Vec<TweetModel>> {
    let option_tweets: Result<Vec<TweetModel>, sqlx::Error> = sqlx::query_as!(
        TweetModel,
        "SELECT 
        tweet_id,
        username,
        parent_id,
        content,
        created_at,
        reactions,
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

use std::fs::File;
use std::io::prelude::*;
pub async fn timeline_for_user(
    username: String,
    data: &web::Data<AppState>,
) -> Option<Vec<TimelineTweets>> {
    let mut result = Vec::new();
    println!("entered timeline_for_user...");
    let user_id: Result<UserId, sqlx::Error> = sqlx::query_as!(
        UserId,
        "SELECT user_id FROM USERS WHERE USERS.username = ?",
        username.clone()
    )
    .fetch_one(&data.db)
    .await;
    let user_id = user_id.unwrap().user_id as usize;

    let option_implicit_ratings: Result<Vec<ImplicitRating>, sqlx::Error> = sqlx::query_as(
        "SELECT
        user_id,
        tweet_id
        FROM REACTIONS, USERS WHERE REACTIONS.username = USERS.username;",
    )
    .fetch_all(&data.db)
    .await;
    let tweets_without_reactions: Result<Vec<(i32,)>, _> =
        sqlx::query_as::<_, (i32,)>("SELECT tweet_id FROM TWEETS WHERE TWEETS.reactions = 0;")
            .fetch_all(&data.db)
            .await;
    for id in tweets_without_reactions.unwrap() {
        result.push(TimelineTweets {
            tweet_id: id.0,
            rating: 0,
        })
    }
    let mut csv_content = String::new();
    csv_content.push_str("user_id,item_id\n");
    let implicit_ratings = option_implicit_ratings.unwrap();
    for rating in &implicit_ratings {
        let rating_csv = format!("{},{}\n", rating.user_id, rating.tweet_id);
        csv_content.push_str(&rating_csv);
    }
    // Create a temporary file
    let temp_file = String::from("implicit-ratings-") + &username + ".csv";
    let temp_file = temp_file.as_str();
    let mut file = File::create(temp_file).expect("Failed to create temporary file");
    // Write the CSV content to the file
    file.write_all(csv_content.as_bytes())
        .expect("Failed to write CSV content to file");
    // Pass the file to the recommendations function

    let file = File::open(temp_file).unwrap();
    let actual = rucommender::recommendations(file).unwrap();
    let recommendations = actual.get(&(user_id as u32)).unwrap();
    // Delete the temporary file
    std::fs::remove_file(temp_file).expect("Failed to delete temporary file");
    for (key, value) in recommendations.iter() {
        result.push(TimelineTweets {
            tweet_id: *key as i32,
            rating: *value as i32,
        })
        // println!("key: {:?}, value: {:?}", key, value);
    }

    Option::from(result)
}

pub async fn all_tweets_replying2_tweetid(
    tweet_id: i32,
    data: &web::Data<AppState>,
) -> Option<Vec<TweetModel>> {
    let option_tweets: Result<Vec<TweetModel>, sqlx::Error> = sqlx::query_as!(
        TweetModel,
        "SELECT 
        tweet_id,
        username,
        parent_id,
        content,
        created_at,
        reactions,
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
            reactions,
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
            let tweets = match tweet_chain {
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

pub async fn tweet_from_tweet_id(tweet_id: i32, data: &web::Data<AppState>) -> Option<TweetModel> {
    let option_tweet: Result<TweetModel, sqlx::Error> = sqlx::query_as!(
        TweetModel,
        "SELECT 
        tweet_id,
        username,
        parent_id,
        content,
        created_at,
        reactions,
        retweets,
        quotes,
        views,
        replies,
        quote_id
    FROM TWEETS
    WHERE
    tweet_id = ?;",
        tweet_id
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

pub async fn tweets_from_username_of_reqtype(
    username: &str,
    tweet_req_type: TweetRequestType,
    data: &web::Data<AppState>,
) -> Vec<TweetModel> {
    let tweets: Vec<TweetModel> = match tweet_req_type {
        TweetRequestType::Tweets => {
            sqlx::query_as!(
                TweetModel,
                r#"
                SELECT 
                    tweet_id,
                    username,
                    parent_id,
                    content, 
                    created_at,
                    reactions,
                    quote_id,
                    quotes,
                    replies,
                    retweets,
                    views
                FROM TWEETS
                WHERE 
                username = ? 
                ORDER BY
                created_at DESC;
                "#,
                username
            )
            .fetch_all(&data.db)
            .await
            .unwrap()
        }
        TweetRequestType::TweetsWithReplies => {
            sqlx::query_as!(
                TweetModel,
                r#"
                SELECT 
                    tweet_id,
                    username,
                    parent_id,
                    content, 
                    created_at,
                    reactions,
                    quote_id,
                    quotes,
                    replies,
                    retweets,
                    views
                FROM TWEETS
                WHERE 
                username = ? AND replies IS NOT NULL
                ORDER BY
                created_at DESC;
                "#,
                username
            )
            .fetch_all(&data.db)
            .await
            .unwrap()
        }
        TweetRequestType::Likes => {
            sqlx::query_as!(
                TweetModel,
                r#"
                SELECT 
                    tweet_id,
                    username,
                    parent_id,
                    content, 
                    created_at,
                    reactions,
                    quote_id,
                    quotes,
                    replies,
                    retweets,
                    views
                FROM TWEETS
                WHERE 
                username = ?
                AND
                tweet_id IN (
                    SELECT
                        tweet_id
                    FROM REACTIONS R
                    WHERE
                        R.username = ?
                )
                ORDER BY
                created_at DESC;
                "#,
                username,
                username
            )
            .fetch_all(&data.db)
            .await
            .unwrap()
        }
        _ => Vec::new(),
    };

    tweets
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
        reactions,
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


pub async fn tweet_with_tweetid_username(_username: &str , tweet_id: i32, data: &web::Data<AppState>) -> Result<TweetModel, sqlx::Error> {
    
    let tweet = sqlx::query_as!(
        TweetModel,
        "
    SELECT 
        tweet_id,
        username,
        parent_id,
        content, 
        created_at,
        reactions,
        quote_id,
        quotes,
        replies,
        retweets,
        views
    FROM TWEETS
    WHERE 
    tweet_id = ?;",
        tweet_id
    )
    .fetch_one(&data.db)
    .await;

    return tweet;
}