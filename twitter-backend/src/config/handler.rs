use actix_web::web;
use crate::routes::auth_routes::{register_post, login_post, logout};
use crate::routes::{
    about_routes::about, 
    auth_routes::{login, register, allusers}, 
    index_routes::index, 
    timeline_routes::timeline,
    profile_routes::{tweets, tweets_images, tweets_likes, tweets_with_replies},
    message_routes::messages,
    notification_routes::notifications,
    bookmark_routes::bookmarks,
    explore_routes::explore,
    tweet_routes::{view_tweet, view_tweet_likes, view_quote_tweets, tweet_analytics}
};

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("")
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
    .service(allusers)
    .service(register_post)
    .service(login_post)
    .service(logout);
    conf.service(scope);
}