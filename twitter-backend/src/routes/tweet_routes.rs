use crate::authentication::middleware::user_exists;
use crate::errors::auth::AuthError;
use crate::{config::AppState, functions::user};
use crate::responses::tweet::{make_tweet_model_response, TweetModelResponse};

use crate::schema::tweet::TweetModel;
use actix_web::{get, web, HttpRequest, HttpResponse};
use sqlx::Row;

#[get("/twitter/{username}/status/{tweetid}")]
pub async fn view_tweet(req: HttpRequest, data: web::Data<AppState>) -> HttpResponse {
    let parts: Vec<&str> = req.path().split('/').collect();
    let username: String = parts[2].to_string();
    let tweet_id: i32 = parts[4].to_string().parse().unwrap();
    println!("username: {:?}", username);
    println!("tweet_id: {:?}", tweet_id);
    let exists: bool = sqlx::query("SELECT EXISTS(SELECT 1 FROM USERS, TWEETS WHERE USERS.username = TWEETS.username AND TWEETS.tweet_id = ? AND TWEETS.username = ?);")
        .bind(tweet_id)
        .bind(username)
        .fetch_one(&data.db)
        .await
        .unwrap()
        .get(0);
    // If the user already exists, return a Conflict response
    if !exists {
        return HttpResponse::Conflict().json(
            serde_json::json!({"status": "fail","message": "This combination of username and tweetid does not exist in database."}),
        );
    }
    let tweet: TweetModel = sqlx::query_as!(
        TweetModel,
        "
    SELECT 
        tweet_id,
        username,
        parent_id,
        content, 
        created_at,
        likes,
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
    .await
    .unwrap();
    let json_response = serde_json::json!({
        "data": {
            "tweet": serde_json::json!({
                "tweet": make_tweet_model_response(&tweet)
            })
        }
    });
    HttpResponse::Ok().json(json_response)
}


#[get("/twitter/{username}/tweets/all")]
pub async fn view_tweet_user(req: HttpRequest, data: web::Data<AppState>) -> HttpResponse {
    let parts: Vec<&str> = req.path().split('/').collect();
    let username: String = parts[2].to_string();
    println!("username: {:?}", username);
    match user_exists(username.clone(),"foo@foo.com".to_string(), &data).await{
        AuthError::UserExistsError => {
            let tweets: Vec<TweetModel> = sqlx::query_as!(
                TweetModel,
                "
            SELECT 
                tweet_id,
                username,
                parent_id,
                content, 
                created_at,
                likes,
                quote_id,
                quotes,
                replies,
                retweets,
                views
            FROM TWEETS
            WHERE 
            username = ?;",
                username
            )
            .fetch_all(&data.db)
            .await
            .unwrap();
        let tweet_responses = tweets
        .into_iter()
        .map(|tweet| make_tweet_model_response(&tweet))
        .collect::<Vec<TweetModelResponse>>();
        let json_response = serde_json::json!({
            "status": "success",
            "results": tweet_responses.len(),
            "tweets": tweet_responses
        });
        HttpResponse::Ok().json(json_response)
        }
        _ => {
            println!("user {} does not exist", username);
            return HttpResponse::Conflict().json(
                serde_json::json!({"status": "fail","message": "This user does not exist in database."}),
            );
        }
    }
}