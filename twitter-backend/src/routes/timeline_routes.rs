use actix_web::{get, HttpResponse};

#[get("/home")]
pub async fn timeline() -> HttpResponse {
    HttpResponse::Ok().body("This will soon be the timeline page!")
}