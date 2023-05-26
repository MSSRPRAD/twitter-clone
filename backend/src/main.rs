use actix_web::{get, App, HttpResponse, HttpServer, Responder};
// use sqlx::{mysql::MySqlPool, Pool};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    HttpServer::new(
        move || {
            App::new()
            .service(index)
        }
    )
    .bind("127.0.0.1:8000")?
    .run()
    .await

}