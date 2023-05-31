use crate::authentication::middleware;
use crate::config::AppState;
use crate::responses::tweet::{make_tweet_model_response};

use crate::schema::tweet::TweetCreateResponse;
use crate::schema::{tweet::TweetModel};
use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse};
use sqlx::Row;

#[get("/twitter/{username}/status/{tweetid}")]
pub async fn view_tweet(req: HttpRequest, data: web::Data<AppState>) -> HttpResponse {
    let parts: Vec<&str> = req.path().split('/').collect();
    let username: String = parts[2].to_string();
    let tweet_id: i32 = parts[4].to_string().parse().unwrap();
    let exists: bool = sqlx::query("SELECT EXISTS(SELECT 1 FROM USERS, TWEETS WHERE USERS.user_id = TWEETS.user_id AND TWEETS.tweet_id = ? AND USERS.username = ?);")
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
        user_id,
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
    tweet_id = ?",
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

#[get("/twitter/{username}/status/{tweetid}/likes")]
pub async fn view_tweet_likes() -> HttpResponse {
    HttpResponse::Ok().body("This will soon be the view tweet likes page!")
}

#[get("/twitter/{username}/status/{tweetid}/quotes")]
pub async fn view_quote_tweets() -> HttpResponse {
    HttpResponse::Ok().body("This will soon be the view quote tweets page!")
}

#[get("/twitter/{username}/status/{tweetid}/analytics")]
pub async fn tweet_analytics() -> HttpResponse {
    HttpResponse::Ok().body("This will soon be the tweet analytics page!")
}

#[post("/twitter/maketweet")]
pub async fn make_tweet(
    req: HttpRequest,
    body: web::Json<TweetCreateResponse>,
    data: web::Data<AppState>,
    _: middleware::JwtMiddleware,
) -> HttpResponse {
    let ext = req.extensions();
    let user_id = ext.get::<i32>().unwrap();
    // Insert tweet into db
    let _insert_result = sqlx::query(
        "INSERT INTO TWEETS 
            (user_id, content) 
        VALUES 
            (?, ?);",
    )
    .bind(user_id)
    .bind(body.content.to_string())
    .execute(&data.db)
    .await;
    HttpResponse::Ok().body("This will soon be the make tweet page!")
}
