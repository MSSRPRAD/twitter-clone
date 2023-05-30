use actix_web::{App, HttpServer, web, http::header, middleware::Logger};
use sqlx::mysql::{MySqlPoolOptions};
use actix_cors::Cors;
// Import the AppState type
use twitter_backend::config::{AppState, config::Config};
use dotenv::dotenv;
use env_logger;
use twitter_backend::config::handler;
use actix_session::storage::RedisActorSessionStore;
use actix_session::SessionMiddleware;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
    env_logger::init();

    let config = Config::init();
    let private_key = actix_web::cookie::Key::generate();
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
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            // redis session middleware
            .wrap(
                SessionMiddleware::builder(
                    RedisActorSessionStore::new("127.0.0.1:6379"),
                    private_key.clone(),
                )
                .build(),
            )
            .app_data(web::Data::new(AppState {
                db: pool.clone(),
                env: config.clone(),
            }))
            .configure(handler::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await

}