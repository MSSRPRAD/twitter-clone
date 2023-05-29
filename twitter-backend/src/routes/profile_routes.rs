use actix_web::{get, HttpResponse};

#[get("/twitter/{username}")]
pub async fn tweets() -> HttpResponse {
    HttpResponse::Ok().body("This will soon be the tweets page!")
}

#[get("/twitter/{username}/with_replies")]
pub async fn tweets_with_replies() -> HttpResponse {
    HttpResponse::Ok().body("This will soon be the tweets with replies page!")
}

#[get("/twitter/{username}/images")]
pub async fn tweets_images() -> HttpResponse {
    HttpResponse::Ok().body("This will soon be the image tweets page!")
}

#[get("/twitter/{username}/likes")]
pub async fn tweets_likes() -> HttpResponse {
    HttpResponse::Ok().body("This will soon be the liked tweets page!")
}