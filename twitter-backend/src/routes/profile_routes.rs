use crate::{
    config::AppState,
    errors::{auth::ErrorResponse, profile::ProfileError},
    functions::{profile::{profile_from_username, create_or_update_profile}, user::user_from_username},
    responses::profile::{make_user_details_response, ProfileModelResponse, make_profile_model_response}, schema::profile::ProfileModel,
};
use actix_web::{get, web, HttpRequest, HttpResponse, Responder, post};
use serde_json::json;
use actix_session::Session;
use crate::schema::user::UserModel;
use crate::authentication::middleware::SessionValue;

#[get("/profile/{username}")]
pub async fn profile_username(
    req: HttpRequest,
    data: web::Data<AppState>,
    // _: middleware::JwtMiddleware,
) -> HttpResponse {
    let temp = req.uri().to_string();
    let username = temp.split("/").last().unwrap();
    // println!("username: {:?}", username[4]);
    let opt_user = user_from_username(username.clone().to_string(), &data).await;
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
                    let details =
                        make_user_details_response(&opt_prof.unwrap(), &opt_user.unwrap());
                    let json_response = json!(details);
                    return HttpResponse::NotFound().json(json_response);
                }
            }
        }
    }
}

#[post("/profile/me")]
pub async fn profile_me(
    body: web::Json<ProfileModelResponse>,
    data: web::Data<AppState>,
    session: Session,
) -> impl Responder {
    println!("reached here");
    let user: Option<SessionValue> = session.get(&"user").unwrap();
    println!("user");
    if let Some(_x) = &user {
        let username = user.unwrap().username;
        let opt_user = user_from_username(username, &data).await;
        match opt_user {
            None => {
                let json_response = json!(ErrorResponse::InvalidUser());
                return HttpResponse::NotFound().json(json_response);
            }
            _ => {
                let _ = create_or_update_profile(body, data).await;
            }
        }
    } else {
        println!("user not logged in");
    }

    HttpResponse::Ok().json(json!({"status": "success"}))
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
