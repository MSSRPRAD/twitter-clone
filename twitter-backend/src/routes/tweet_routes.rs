use crate::authentication::middleware::{user_exists, SessionValue};
use crate::errors::auth::{AuthError, ErrorResponse};
use crate::functions;
use crate::functions::profile::profile_from_username;
use crate::functions::tweet::{
    create_tweet, most_recent_tweet_from_username, timeline_for_user, tweet_from_tweet_id,
    tweet_quoted, parent_tweet_chain_from_tweetid, all_tweets_replying2_tweetid, tweets_from_username, tweet_with_tweetid_username,
};
use crate::functions::user::user_from_username;
use crate::responses::{
    reaction::ReactionModelResponse,
    tweet::{make_tweet_model_response, CreateTweetModelResponse, TweetModelResponse},
};
use crate::schema::tweet::TweetModel;
use crate::{config::AppState, functions::user};
use actix_session::Session;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use serde_json::json;
use sqlx::{Error, Row};

#[get("/twitter/timeline/me")]
pub async fn timeline_from_username(
    req: HttpRequest,
    data: web::Data<AppState>,
    session: Session,
) -> HttpResponse {
    let user: Option<SessionValue> = session.get(&"user").unwrap();
    println!("user: {:?}", user);
    if let Some(_x) = &user {
        let username = user.unwrap().username;
        let opt_user = user_from_username(username.clone(), &data).await;
        match opt_user {
            None => {
                let json_response = json!(ErrorResponse::InvalidUser());
                return HttpResponse::NotFound().json(json_response);
            }
            _ => {
                println!("username: {:?}", username);
                match user_exists(username.clone(), "foo@foo.com".to_string(), &data).await {
                    AuthError::UserExistsError => {
                        let timeline = timeline_for_user(username, &data).await.unwrap();
                        let mut timeline_tweets = Vec::new();
                        let mut ratings = Vec::new();
                        let mut profile_pics = std::collections::HashMap::new();
                        let mut quoted_tweets = std::collections::HashMap::new();
                        for t in timeline {
                            let tweet = tweet_from_tweet_id(t.tweet_id, &data).await.unwrap();
                            timeline_tweets.push(tweet.clone());
                            ratings.push(t.rating);
                            profile_pics.insert(t.tweet_id, {
                                let prf = profile_from_username(tweet.clone().username, &data).await;
                                let ret;
                                match prf {
                                    Some(pic) => {
                                        ret = Some(pic.profilepicurl.unwrap());
                                    }
                                    None => {
                                        ret = None;
                                    }
                                }
                                ret
                            });
                        }
                        timeline_tweets.sort_by_key(|tweet| ratings[tweet.tweet_id as usize]);
                        let timeline_tweet_responses = timeline_tweets
                            .into_iter()
                            .map(|tweet| make_tweet_model_response(&tweet))
                            .collect::<Vec<TweetModelResponse>>();
                        // println!("quoted tweets: {:?}", quoted_tweets);
                        for tweet in &mut timeline_tweet_responses
                            .iter()
                            .filter(|tweet| tweet.quote_id.is_some())
                        {
                            let quoted_tweet =
                                tweet_from_tweet_id(tweet.quote_id.unwrap(), &data).await;
                            match quoted_tweet {
                                Some(quoted_tweet) => {
                                    profile_pics.insert(quoted_tweet.tweet_id, {
                                        let prf = profile_from_username(
                                            quoted_tweet.clone().username,
                                            &data,
                                        )
                                        .await;
                                        let ret;
                                        match prf {
                                            Some(pic) => {
                                                ret = Some(pic.profilepicurl.unwrap());
                                            }
                                            None => {
                                                ret = None;
                                            }
                                        }
                                        ret
                                    });
                                    quoted_tweets.insert(
                                        quoted_tweet.tweet_id,
                                        make_tweet_model_response(&quoted_tweet),
                                    );
                                }
                                None => {
                                    println!("quoted tweet does not exist");
                                }
                            }
                        }
                        // println!("profile_pics: {:?}", profile_pics);
                        let json_response = serde_json::json!({
                            "profile_urls": profile_pics,
                            "status": "success",
                            "results": timeline_tweet_responses.len(),
                            "ratings": ratings,
                            "tweets": timeline_tweet_responses,
                            "quoted_tweets": quoted_tweets,
                        });
                        // println!("{:?}", json_response);
                        return HttpResponse::Ok().json(json_response);
                    }
                    _ => {
                        // println!("user {} does not exist", username);
                        return HttpResponse::Conflict().json(
                            serde_json::json!({"status": "fail","message": "This user does not exist in database."}),
                        );
                    }
                }
            }
        }
    } else {
        let json_response = json!(ErrorResponse::NotLoggedIn());
        return HttpResponse::Unauthorized().json(json_response);
    }
}

#[get("/twitter/{username}/status/{tweetid}")]
pub async fn view_tweet(req: HttpRequest, data: web::Data<AppState>) -> HttpResponse {
    let parts: Vec<&str> = req.path().split('/').collect();
    let username: String = parts[2].to_string();
    let tweet_id: i32 = parts[4].to_string().parse().unwrap();
    println!("username: {:?}", username);
    println!("tweet_id: {:?}", tweet_id);
    let tweet = tweet_with_tweetid_username(&username, tweet_id, &data).await;
    // If the user already exists, return a Conflict response
    match tweet {
        Result::Err(err) => {
            return HttpResponse::Conflict().json(
                serde_json::json!({"status": "fail","message": "This combination of username and tweetid does not exist in database."}),
            );
        },
        Ok(tweet) => {
            let json_response = serde_json::json!({
                "data": {
                    "tweet": serde_json::json!({
                        "tweet": make_tweet_model_response(&tweet)
                    })
                }
            });
            HttpResponse::Ok().json(json_response)
        }
    }
    
}

#[get("/twitter/{tweetid}/quoted")]
pub async fn get_quoted(req: HttpRequest, data: web::Data<AppState>) -> HttpResponse {
    let parts: Vec<&str> = req.path().split('/').collect();
    let tweet_id: i32 = parts[2].parse().unwrap();
    let tweet = tweet_from_tweet_id(tweet_id, &data).await;
    match tweet {
        Some(tweet) => match tweet.quote_id {
            Some(quote_id) => {
                let quoted_tweet = tweet_from_tweet_id(quote_id, &data).await;
                match quoted_tweet {
                    Some(quoted_tweet) => {
                        let json_response = serde_json::json!({
                            "data": {
                                "tweet": make_tweet_model_response(&quoted_tweet)
                            }
                        });
                        HttpResponse::Ok().json(json_response)
                    }
                    None => {
                        return HttpResponse::NoContent().json(
                                serde_json::json!({"status": "fail","message": "This tweet does not exist."}),
                            );
                    }
                }
            }
            None => {
                return HttpResponse::NoContent().json(
                    serde_json::json!({"status": "fail","message": "This tweet has no quotes."}),
                );
            }
        },
        None => {
            return HttpResponse::NoContent().json(
                serde_json::json!({"status": "fail","message": "This tweet does not exist."}),
            );
        }
    }
}

#[get("/twitter/{username}/tweets/all")]
pub async fn view_tweet_user(req: HttpRequest, data: web::Data<AppState>) -> HttpResponse {
    let parts: Vec<&str> = req.path().split('/').collect();
    let username: String = parts[2].to_string();
    println!("username: {:?}", username);
    match user_exists(username.clone(), "foo@foo.com".to_string(), &data).await {
        AuthError::UserExistsError => {
            let tweets: Vec<TweetModel> = tweets_from_username(&username, &data).await;
            let mut tweet_responses = tweets
                .into_iter()
                .map(|tweet| make_tweet_model_response(&tweet))
                .collect::<Vec<TweetModelResponse>>();
            // println!("quoted tweets: {:?}", quoted_tweets);
            let mut quoted_tweets = std::collections::HashMap::new();
            let mut profile_pics = std::collections::HashMap::new();
            for tweet in &mut tweet_responses
                .iter()
                .filter(|tweet| tweet.quote_id.is_some())
            {
                let quoted_tweet = tweet_from_tweet_id(tweet.quote_id.unwrap(), &data).await;
                match quoted_tweet {
                    Some(quoted_tweet) => {
                        profile_pics.insert(quoted_tweet.tweet_id, {
                            // println!("fetching profile_pic for user: {:?}", quoted_tweet.clone().username);
                            let prf =
                                profile_from_username(quoted_tweet.clone().username, &data).await;
                            let ret;
                            match prf {
                                Some(prof) => {
                                    ret = Some(prof.profilepicurl);
                                }
                                _ => {
                                    ret = None;
                                }
                            }
                            ret
                        });
                        quoted_tweets.insert(
                            quoted_tweet.tweet_id,
                            make_tweet_model_response(&quoted_tweet),
                        );
                    }
                    None => {
                        println!("quoted tweet does not exist");
                    }
                }
            }
            let json_response = serde_json::json!({
                "profile_pics": profile_pics,
                "status": "success",
                "results": tweet_responses.len(),
                "tweets": tweet_responses,
                "quoted_tweets": quoted_tweets,
            });
            println!("{:?}", json_response);
            HttpResponse::Ok().json(json_response)
        }
        _ => {
            println!("user {} does not exist", username);
            return HttpResponse::Conflict().json(
                serde_json::json!({"status": "fail","message": "This user does not exist in database."}),
            );
        }
    }
}

#[get("/twitter/{tweet_id}/tweetchain")]
pub async fn tweet_chain_tweetid(req: HttpRequest, data: web::Data<AppState>) -> HttpResponse {
    let parts: Vec<&str> = req.path().split('/').collect();
    let tweet_id: i32 = parts[2].to_string().parse().unwrap();
    match tweet_from_tweet_id(tweet_id, &data).await {
        Some(tweet1) => {
            let tweet_curr = tweet1.clone();
            let parent_chain: Vec<TweetModel> = parent_tweet_chain_from_tweetid(tweet_id, None, &data).await.unwrap().unwrap();
            let replies: Vec<TweetModel> = all_tweets_replying2_tweetid(tweet_id, &data).await.unwrap();
            let mut parent_chain_tweet_responses = parent_chain
                .into_iter()
                .map(|tweet| make_tweet_model_response(&tweet))
                .collect::<Vec<TweetModelResponse>>();
            let mut replies_tweet_responses = replies
                .into_iter()
                .map(|tweet| make_tweet_model_response(&tweet))
                .collect::<Vec<TweetModelResponse>>();
            // println!("quoted tweets: {:?}", quoted_tweets);
            let mut quoted_tweets = std::collections::HashMap::new();
            let mut profile_pics = std::collections::HashMap::new();
            profile_pics.insert(
                tweet1.tweet_id,
                {
                    let prf = profile_from_username(tweet1.username, &data).await;
                    let res;
                    match prf {
                        Some(prof) => {res = Some(prof.profilepicurl);}
                        None => {res = None}
                    }
                    res
                }
            );
            for tweet in &mut parent_chain_tweet_responses
                .iter()
                .chain(
                    replies_tweet_responses.iter()
                )
                .filter(|tweet| tweet.quote_id.is_some())
            {
                let quoted_tweet = tweet_from_tweet_id(tweet.quote_id.unwrap(), &data).await;
                match quoted_tweet {
                    Some(quoted_tweet) => {
                        profile_pics.insert(quoted_tweet.tweet_id, {
                            // println!("fetching profile_pic for user: {:?}", quoted_tweet.clone().username);
                            let prf =
                                profile_from_username(quoted_tweet.clone().username, &data).await;
                            let ret;
                            match prf {
                                Some(prof) => {
                                    ret = Some(prof.profilepicurl);
                                }
                                _ => {
                                    ret = None;
                                }
                            }
                            ret
                        });
                        quoted_tweets.insert(
                            quoted_tweet.tweet_id,
                            make_tweet_model_response(&quoted_tweet),
                        );
                    }
                    None => {
                        println!("quoted tweet does not exist");
                    }
                }
            }
            // let mut tweets = Vec::new(); tweets.push(make_tweet_model_response(&tweet_curr));
            let json_response = serde_json::json!({
                "profile_pics": profile_pics,
                "status": "success",
                "results": parent_chain_tweet_responses.len()+replies_tweet_responses.len(),
                "parent_chain": parent_chain_tweet_responses,
                "replies": replies_tweet_responses,
                "quoted_tweets": quoted_tweets,
                "tweet": make_tweet_model_response(&tweet_curr),
            });
            println!("{:?}", json_response);
            HttpResponse::Ok().json(json_response)
        }
        None => {
            println!("Tweet {} does not exist", tweet_id);
            return HttpResponse::Conflict().json(
                serde_json::json!({"status": "fail","message": "This Tweet does not exist in database."}),
            );
        }
    }
}



#[post("/tweets/reactions/{tweet_id}")]
pub async fn tweet_reaction(
    body: web::Json<ReactionModelResponse>,
    data: web::Data<AppState>,
    session: Session,
) -> impl Responder {
    println!("reached reaction endpoint");
    let user: Option<SessionValue> = session.get(&"user").unwrap();
    println!("user: {:?}", user);
    if let Some(_x) = &user {
        let username = user.unwrap().username;
        let opt_user = user_from_username(username.clone(), &data).await;
        match opt_user {
            None => {
                let json_response = json!(ErrorResponse::InvalidUser());
                return HttpResponse::NotFound().json(json_response);
            }
            _ => {
                let response = functions::tweet::create_or_remove_reaction(
                    username,
                    body.tweet_id,
                    body.reaction_id,
                    data,
                )
                .await;
                return HttpResponse::Ok().json(json!(response));
            }
        }
    } else {
        let json_response = json!(ErrorResponse::NotLoggedIn());
        return HttpResponse::Unauthorized().json(json_response);
    }
}

#[post("/tweets/me")]
pub async fn tweet_me(
    body: web::Json<CreateTweetModelResponse>,
    data: web::Data<AppState>,
    session: Session,
) -> impl Responder {
    println!("reached create tweet");
    let user: Option<SessionValue> = session.get(&"user").unwrap();
    println!("user: {:?}", user);
    if let Some(_x) = &user {
        let username = user.unwrap().username;
        let opt_user = user_from_username(username.clone(), &data).await;
        match opt_user {
            None => {
                let json_response = json!(ErrorResponse::InvalidUser());
                return HttpResponse::NotFound().json(json_response);
            }
            _ => {
                let _ = create_tweet(body, data.clone(), username.clone()).await;
                let created_tweet = most_recent_tweet_from_username(username, &data).await;
                match created_tweet {
                    None => {
                        return HttpResponse::InternalServerError()
                            .json(json!(ErrorResponse::InternalServerError()));
                    }
                    Some(created_tweet) => {
                        return HttpResponse::Ok()
                            .json(json!(make_tweet_model_response(&created_tweet)))
                    }
                }
            }
        }
    } else {
        let json_response = json!(ErrorResponse::NotLoggedIn());
        return HttpResponse::Unauthorized().json(json_response);
    }
}
