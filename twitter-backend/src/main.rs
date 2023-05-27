use actix_web::{App, HttpServer};
use twitter_backend::handlers::{
    about_handler::about, 
    auth_handler::{login, register}, 
    index_handler::index, 
    timeline_handler::timeline,
    profile_handler::{tweets, tweets_images, tweets_likes, tweets_with_replies},
    message_handler::messages,
    notification_handler::notifications,
    bookmark_handler::bookmarks,
    explore_handler::explore,
    tweet_handler::{view_tweet, view_tweet_likes, view_quote_tweets, tweet_analytics}
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    
    println!("Backend Server started successfully!");
    println!("Serving on 127.0.0.1:8000");

    HttpServer::new(
        move || {
            App::new()
            .service(index)
            .service(about)
            .service(login)
            .service(register)
            .service(timeline)
            .service(tweets)
            .service(tweets_images)
            .service(tweets_likes)
            .service(tweets_with_replies)
            .service(notifications)
            .service(messages)
            .service(bookmarks)
            .service(explore)
            .service(view_tweet)
            .service(view_tweet_likes)
            .service(view_quote_tweets)
            .service(tweet_analytics)
        }
    )
    .bind("127.0.0.1:8000")?
    .run()
    .await

}