use actix_web::{get, HttpResponse};

#[get("/twitter/notifications")]
pub async fn notifications() -> HttpResponse {
    HttpResponse::Ok().body("This will soon be the notifications page!")
}
