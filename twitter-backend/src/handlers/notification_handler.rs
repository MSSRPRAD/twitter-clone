use actix_web::{get, HttpResponse};

#[get("/notifications")]
pub async fn notifications() -> HttpResponse {
    HttpResponse::Ok().body("This will soon be the notifications page!")
}