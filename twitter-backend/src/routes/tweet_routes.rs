use crate::authentication::middleware::{user_exists, SessionValue};
use crate::errors::auth::{AuthError, ErrorResponse};
use crate::functions::tweet::create_tweet;
use crate::functions::user::user_from_username;
use crate::{config::AppState, functions::user};
use crate::responses::tweet::{make_tweet_model_response, TweetModelResponse, CreateTweetModelResponse};

use crate::schema::tweet::TweetModel;
use actix_session::Session;
use actix_web::{get, web, HttpRequest, HttpResponse, post, Responder};
use serde_json::json;
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

#[post("/tweets/me")]
pub async fn tweet_me(
    body: web::Json<CreateTweetModelResponse>,
    data: web::Data<AppState>,
    session: Session,
) -> impl Responder {
    println!("reached here");
    let user: Option<SessionValue> = session.get(&"user").unwrap();
    // println!("user");
    if let Some(_x) = &user {
        let username = user.unwrap().username;
        let opt_user = user_from_username(username, &data).await;
        match opt_user {
            None => {
                let json_response = json!(ErrorResponse::InvalidUser());
                return HttpResponse::NotFound().json(json_response);
            }
            _ => {
                let _ = create_tweet(body, data).await;
            }
        }
    } else {
       let json_response = json!(ErrorResponse::NotLoggedIn());
       return HttpResponse::Unauthorized().json(json_response);
    }

    HttpResponse::Ok().json(json!({"status": "success"}))
}