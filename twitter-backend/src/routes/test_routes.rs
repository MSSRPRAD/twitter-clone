use actix_web::{get, web, HttpRequest, HttpResponse};
use serde_json::json;

use crate::{
    config::AppState,
    functions::{self, tweet::timeline_for_user},
};

#[get("/test/{username}")]
pub async fn test_route(req: HttpRequest, data: web::Data<AppState>) -> HttpResponse {
    let temp = req.uri().to_string();
    let username = temp.split("/").last().unwrap();
    // println!("username: {:?}", username);
    let timeline_tweets = timeline_for_user(username.to_string(), &data).await;
    println!("timeline_tweets: {:?}", timeline_tweets);
    HttpResponse::Ok().body(format!("This is a test page: {:?}", "nothing"))
}
