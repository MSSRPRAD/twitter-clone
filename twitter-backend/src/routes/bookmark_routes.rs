use actix_web::{get, HttpResponse};

#[get("/twitter/bookmarks")]
pub async fn bookmarks() -> HttpResponse {
    HttpResponse::Ok().body("This will soon be the bookmarks page!")
}