use actix_web::{get, HttpResponse};

#[get("/twitter/messages")]
pub async fn messages() -> HttpResponse {
    HttpResponse::Ok().body("This will soon be the messages page!")
}