use actix_web::{get, HttpResponse};

#[get("/messages")]
pub async fn messages() -> HttpResponse {
    HttpResponse::Ok().body("This will soon be the messages page!")
}