use actix_web::{get, HttpResponse};

#[get("/twitter/explore")]
pub async fn explore() -> HttpResponse {
    HttpResponse::Ok().body("This will soon be the explore page!")
}