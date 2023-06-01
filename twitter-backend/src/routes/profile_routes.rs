use crate::{config::AppState, functions::{user::{self, user_from_username}, profile::profile_from_username}, errors::{profile::ProfileError, auth::ErrorResponse}, responses::profile::make_user_details_response};
use actix_web::{get, web, HttpRequest, HttpResponse};
use serde_json::{Value, json};
use crate::responses::profile::UserDetailsResponse;
#[get("/profile/{username}")]
pub async fn profile_username(
    req: HttpRequest,
    data: web::Data<AppState>,
    // _: middleware::JwtMiddleware,
) -> HttpResponse {
    let username = req.query_string().split("/").last().unwrap();
    let opt_user = user_from_username(username.to_string(), &data).await;
    match opt_user {
        None => {
            let json_response = json!(ErrorResponse::InvalidUser());
            return HttpResponse::NotFound().json(json_response);
        }
        _ => {
            let opt_prof = profile_from_username(username.to_string(), &data).await;
            match opt_prof {
                None => {
                    let json_response = json!(ErrorResponse::NoProfile());
                    return HttpResponse::NotFound().json(json_response);
                }
                _ => {
                    let details = make_user_details_response(&opt_prof.unwrap(), &opt_user.unwrap());
                    let json_response = json!(ErrorResponse::NoError());
                    return HttpResponse::NotFound().json(json_response);
                }
            }
        }
    }
}

#[get("/twitter/{username}")]
pub async fn tweets() -> HttpResponse {
    HttpResponse::Ok().body("This will soon be the tweets page!")
}

#[get("/twitter/{username}/with_replies")]
pub async fn tweets_with_replies() -> HttpResponse {
    HttpResponse::Ok().body("This will soon be the tweets with replies page!")
}

#[get("/twitter/{username}/images")]
pub async fn tweets_images() -> HttpResponse {
    HttpResponse::Ok().body("This will soon be the image tweets page!")
}

#[get("/twitter/{username}/likes")]
pub async fn tweets_likes() -> HttpResponse {
    HttpResponse::Ok().body("This will soon be the liked tweets page!")
}
