use crate::routes::auth_routes::{get_me_handler, login_post, logout, register_post};
use crate::routes::{
    about_routes::about,
    auth_routes::{allusers, login, register},
    bookmark_routes::bookmarks,
    explore_routes::explore,
    index_routes::index,
    message_routes::messages,
    notification_routes::notifications,
    profile_routes::{profile_username, tweets, tweets_images, tweets_likes, tweets_with_replies},
    timeline_routes::timeline,
    tweet_routes::{make_tweet, tweet_analytics, view_quote_tweets, view_tweet, view_tweet_likes},
};
use actix_web::web;

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
        .service(get_me_handler)
        .service(profile_username)
        .service(make_tweet)
        .service(logout);
    conf.service(scope);
}
