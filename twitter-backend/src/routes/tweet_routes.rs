use actix_web::{get, HttpResponse};

#[get("/{username}/status/{tweetid}")]
pub async fn view_tweet() -> HttpResponse {
    HttpResponse::Ok().body("This will soon be the view tweet page!")
}

#[get("/{username}/status/{tweetid}/likes")]
pub async fn view_tweet_likes() -> HttpResponse {
    HttpResponse::Ok().body("This will soon be the view tweet likes page!")
}

#[get("/{username}/status/{tweetid}/quotes")]
pub async fn view_quote_tweets() -> HttpResponse {
    HttpResponse::Ok().body("This will soon be the view quote tweets page!")
}

#[get("/{username}/status/{tweetid}/analytics")]
pub async fn tweet_analytics() -> HttpResponse {
    HttpResponse::Ok().body("This will soon be the tweet analytics page!")
}