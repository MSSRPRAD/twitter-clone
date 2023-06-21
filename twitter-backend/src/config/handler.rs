use crate::routes::auth_routes::{get_me_handler, login_post, logout, register_post};
use crate::routes::follow_routes::{user_me};
use crate::routes::test_routes::test_route;
use crate::routes::tweet_routes::{view_tweet_user, tweet_me};
use crate::routes::{
    auth_routes::{allusers, login, register},
    profile_routes::{
        profile_me, profile_username, tweets, tweets_images, tweets_likes, tweets_with_replies,
    },
    tweet_routes::view_tweet,
    follow_routes::follow_username,
};
use actix_web::web;

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("")
        .service(login)
        .service(register)
        .service(tweets)
        .service(tweets_images)
        .service(tweets_likes)
        .service(tweets_with_replies)
        .service(view_tweet)
        .service(allusers)
        .service(register_post)
        .service(login_post)
        .service(get_me_handler)
        .service(profile_username)
        .service(profile_me)
        .service(follow_username)
        .service(user_me)
        .service(view_tweet_user)
        .service(tweet_me)
        .service(test_route)
        .service(logout);
    conf.service(scope);
}
