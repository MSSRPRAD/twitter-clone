use crate::{functions::user::user_from_username, functions::following::create_or_update_following};
use crate::errors::auth::{ErrorResponse, AuthError};
use actix_session::Session;
use crate::config::AppState;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use serde_json::json;
use crate::authentication::middleware::SessionValue;
use crate::responses::following::FollowingModelResponse;

#[get("/follow/{username}")]
async fn follow_username(
    req: HttpRequest,
    data: web::Data<AppState>,
    session: Session,
) -> impl Responder {
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