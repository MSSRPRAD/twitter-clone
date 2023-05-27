use actix_web::{get, HttpResponse};

#[get("/about")]
pub async fn about() -> HttpResponse {
    HttpResponse::Ok().body("This will soon be an about page!")
}