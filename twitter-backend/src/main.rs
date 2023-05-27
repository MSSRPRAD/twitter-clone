use actix_web::{App, HttpServer, web, middleware::Logger};
use sqlx::mysql::{MySqlPoolOptions};
// Import the AppState type
use twitter_backend::appstate::{AppState, handler};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    println!("🚀 Server started successfully");
    println!("Serving on 127.0.0.1:8000");

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(AppState { db: pool.clone() }))
        .configure(handler::config)
        .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await

}