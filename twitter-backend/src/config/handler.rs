use crate::routes::auth_routes::{get_me_handler, login_post, logout, register_post};
use crate::routes::follow_routes::user_me;
use crate::routes::test_routes::test_route;
use crate::routes::tweet_routes::{get_quoted, timeline_from_username, tweet_me, view_tweet_user};
use crate::routes::{
    auth_routes::{login, register},
    follow_routes::follow_username,
    profile_routes::{
        profile_me, profile_username,
    },
    tweet_routes::{
        view_tweet,
        tweet_chain_tweetid,
    },
    test_routes::{allusers}
};
use actix_web::web;

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("")
        .service(login)
        .service(register)
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
        .service(get_quoted)
        .service(tweet_chain_tweetid)
        .service(timeline_from_username)
        .service(logout);
    conf.service(scope);
}
