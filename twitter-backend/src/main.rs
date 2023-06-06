use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web, App, HttpServer};
use sqlx::mysql::MySqlPoolOptions;
// Import the AppState type
use actix_session::storage::RedisActorSessionStore;
use actix_session::SessionMiddleware;
use dotenv::dotenv;
use env_logger;
use twitter_backend::config::handler;
use twitter_backend::config::{config::Config, AppState};
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

    // let redis_url = std::env::var("REDIS_URI").expect("Failed to get REDIS_URL.");

    // Redis connection pool
    // let cfg = deadpool_redis::Config::from_url(redis_url.clone());
    // let redis_pool = cfg
    //     .create_pool(Some(deadpool_redis::Runtime::Tokio1))
    //     .expect("🔥 Failed to connect to the redis instance.");
    // let redis_pool_data = actix_web::web::Data::new(redis_pool);
    // println!("✅Connection to the redis instance is successful!");
    // // For session
    // // Random secret key
    // let secret_key = actix_web::cookie::Key::from(&[0; 100]);
    // let redis_store = actix_session::storage::RedisSessionStore::new(redis_url.clone())
    //     .await
    //     .expect("Cannot unwrap redis session.");

    println!("🚀 Server started successfully");
    println!("Serving on 127.0.0.1:8000");

    HttpServer::new(move || {
        let cors = Cors::permissive();
            // .supports_credentials();
        App::new()
            // redis session middleware
            .wrap(
                SessionMiddleware::builder(
                    RedisActorSessionStore::new("127.0.0.1:6379"),
                    private_key.clone(),
                )
                .cookie_name("test-session".to_string())
                .build(),
            )
            .app_data(web::Data::new(AppState {
                db: pool.clone(),
                env: config.clone(),
                // sessiondb: redis_pool_data.clone(),
            }))
            .configure(handler::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
