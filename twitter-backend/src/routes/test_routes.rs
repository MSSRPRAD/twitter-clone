use actix_web::{HttpResponse, HttpRequest, get, web};
use serde_json::json;

use crate::{functions, config::AppState};

#[get("/test/{tweet_id}")]
pub async fn test_route(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> HttpResponse {
    let temp = req.uri().to_string();
    let tweet_id = temp.split("/").last().unwrap();
    // println!("{:?}", tweet_id);
    let tweet_chain = functions::tweet::parent_tweet_chain_from_tweetid(tweet_id.parse::<i32>().unwrap(), None, &data).await;
    // println!("{:?}", tweet_chain);
    HttpResponse::Ok().body(format!("This will soon be the tweets page! {:?}", tweet_chain))
}