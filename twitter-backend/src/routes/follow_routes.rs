use crate::functions::following::get_following_details_response;
use crate::{functions::user::user_from_username, functions::following::create_or_update_following};
use crate::errors::auth::{ErrorResponse, AuthError};
use actix_session::Session;
use crate::config::AppState;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder, post};
use serde_json::json;
use crate::authentication::middleware::SessionValue;
use crate::responses::following::{FollowingModelResponse, FollowingDetailsResponse};

#[get("/follow/{username}")]
async fn follow_username(
    req: HttpRequest,
    data: web::Data<AppState>,
    session: Session,
) -> impl Responder {
    println!("reached here!");
    let temp = req.uri().to_string();
    let username = temp.split("/").last().unwrap();
    println!("username: {:?}", username);
    let user: Option<SessionValue> = session.get(&"user").unwrap();
    println!("logged in: {:?}", user);
    if let Some(_x) = &user {
        let req_username = &user.unwrap().username;
        match user_from_username(req_username.to_string(), &data).await{
            None => {
                let response_json = serde_json::json!(ErrorResponse::InvalidUser());
                return HttpResponse::NotFound().json(response_json);
            }
            _ => {
                match user_from_username(username.to_string(), &data).await {
                    None => {
                        let response_json = serde_json::json!(ErrorResponse::InvalidUser());
                        return HttpResponse::NotFound().json(response_json);
                    }
                    _ => {
                        // Add the follow relationship to database
                        let following = FollowingModelResponse {
                            username: req_username.to_string(),
                            following: username.to_string(),
                        };
                        let _ = create_or_update_following(following, data).await;
                        let json_response = json!(ErrorResponse::NoError());
                        return HttpResponse::Ok().json(json_response);
                    }
                }
            }
        }
    } else {
        let response_json = json!(ErrorResponse::NotLoggedIn());
        return HttpResponse::Unauthorized().json(response_json);
    } 

}

#[get("/followdetails/{username}")]
pub async fn user_me(
    data: web::Data<AppState>,
    session: Session,
    req: HttpRequest,
) -> impl Responder {
    println!("reached here");
    let user: Option<SessionValue> = session.get(&"user").unwrap();
    println!("user:{:?}", user);
    let temp = req.uri().to_string();
    let requested_username = temp.split("/").last().unwrap();
    // println!("user");
    if let Some(_x) = &user {
        let username = user.unwrap().username;
        let opt_user = user_from_username(username.clone(), &data).await;
        match opt_user {
            None => {
                let json_response = json!(ErrorResponse::InvalidUser());
                return HttpResponse::Unauthorized().json(json_response);
            }
            _ => {
                let opt_user1 = user_from_username(requested_username.to_string(), &data).await;
                match opt_user1 {
                    None => {
                        let json_response = json!(ErrorResponse::InvalidUser());
                        return HttpResponse::NotFound().json(json_response);
                    }
                    _ => {
                        let FollowingDetailsResponse = get_following_details_response(requested_username, &username, data).await;
                        let json_response = json!(FollowingDetailsResponse);
                        return HttpResponse::Ok().json(json_response);
                    }
                }
            }
        }
    } else {
       let json_response = json!(ErrorResponse::NotLoggedIn());
       return HttpResponse::Unauthorized().json(json_response);
    }

    HttpResponse::Ok().json(json!({"status": "success"}))
}