
use crate::config::AppState;
use crate::responses::profile::make_profile_model_response;
use crate::responses::user::make_user_model_response;
use crate::schema::{profile::ProfileModel, user::UserModel};
use actix_web::{get, web, HttpRequest, HttpResponse};

#[get("/profile/{username}")]
pub async fn profile_username(
    req: HttpRequest,
    data: web::Data<AppState>,
    // _: middleware::JwtMiddleware,
) -> HttpResponse {
    // let ext = req.extensions();
    // let user_id = ext.get::<i32>().unwrap();
    let username: String = req
        .path()
        .split('/')
        .into_iter()
        .nth(2)
        .unwrap()
        .to_string();
    let user = sqlx::query_as!(
        UserModel,
        "
    SELECT 
        user_id,
        name,
        role_id, 
        username, 
        email, 
        created_at, 
        dob, 
        profile_id, 
        password 
    FROM USERS
    WHERE 
    username = ?",
        username
    )
    .fetch_one(&data.db)
    .await
    .unwrap();
    let profile_id = user.profile_id.unwrap();
    let profile: ProfileModel = sqlx::query_as!(
        ProfileModel,
        "
    SELECT 
        profile_id,
        about,
        phone_no, 
        location, 
        languages
        FROM PROFILES
        WHERE 
    profile_id = (?);",
        profile_id
    )
    .fetch_one(&data.db)
    .await
    .unwrap();
    let json_response = serde_json::json!({
        "status":  "success",
        "data": {
            "profile": serde_json::json!({
                "profile": make_profile_model_response(&profile)
            }),
            "user": serde_json::json!({
                "user": make_user_model_response(&user)
            })
        }
    });
    HttpResponse::Ok().json(json_response)
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
