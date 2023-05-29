use actix_web::{get, HttpResponse, HttpRequest, web};
use crate::config::AppState;
use crate::responses::tweet::make_tweet_model_response;
use crate::schema::{tweet::TweetModel, user::UserModel};
/*
pub tweet_id: i32,
    pub user_id: i32,
    pub parent_id: Option<i32>,
    pub content: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>
 */
#[get("/twitter/{username}/status/{tweetid}")]
pub async fn view_tweet(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> HttpResponse {
    let parts: Vec<&str> = req.path().split('/').collect();
    let username: String = parts[2].to_string();
    let tweet_id: i32 = parts[4].to_string().parse().unwrap();
    let user = sqlx::query_as!(UserModel, "
    SELECT 
        user_id,
        name,
        role_id, 
        username, 
        email, 
        created_at, 
        dob, 
        profile_id, 
        password 
    FROM USERS
    WHERE 
    username = ?", username)
        .fetch_one(&data.db)
        .await
        .unwrap();
    let tweet: TweetModel = sqlx::query_as!(TweetModel, "
    SELECT 
        tweet_id,
        user_id,
        parent_id,
        content, 
        created_at
    FROM TWEETS
    WHERE 
    tweet_id = ?", tweet_id)
        .fetch_one(&data.db)
        .await
        .unwrap();
    if user.user_id != tweet.user_id {
        HttpResponse::InternalServerError();
    }
    println!("reached here");
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