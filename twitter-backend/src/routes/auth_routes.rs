use crate::authentication::middleware::validate_credentials;
use crate::authentication::middleware::JwtMiddleware;
use crate::authentication::middleware::{register_user, user_exists, SessionValue};
use crate::errors::auth::ErrorResponse;
use crate::schema::user::{LoginUserSchema, RegisterUserSchema, UserModel};
use crate::{
    authentication::middleware::{self, TokenClaims},
    errors::auth::AuthError,
};
use crate::{
    config::AppState,
    responses::user::{make_user_model_response, UserModelResponse},
};
use actix_session::storage::SessionKey;
use actix_session::Session;
use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;
use sqlx::Row;

#[post("/login")]
async fn login_post(
    body: web::Json<LoginUserSchema>,
    data: web::Data<AppState>,
    session: Session,
) -> impl Responder {
    // let mut redis_con = middleware::get_redis_con(data.sessiondb.clone()).await;
    let loginuser = body.into_inner();
    let auth_error = validate_credentials(&loginuser, data).await;
    let response_json;
    // If it is not valid, return a BadRequest response
    match auth_error {
        AuthError::InvalidUsernameError => {
            response_json = json!(ErrorResponse::InvalidUser());
        }
        AuthError::WrongPasswordError => {
            response_json = json!(ErrorResponse::InvalidCredentials());
        }
        AuthError::NoError => {
            let value: SessionValue = SessionValue {
                username: loginuser.username.clone(),
                role_id: 0,
                created_at: Some(chrono::Utc::now()),
            };
            let _ = session.insert("user", &value).map_err(|_| {
                println!("could not add user to session");
                return HttpResponse::Ok();
            });
            session.renew();
            response_json = json!(ErrorResponse::NoError());
        }
        _ => {
            response_json = json!(ErrorResponse::NoError());
        }
    }
    // If it is valid, return cookie with the session id
    // Return token with response
    HttpResponse::Ok().json(response_json)
}

#[get("/login")]
pub async fn login() -> HttpResponse {
    HttpResponse::Ok().body("This will soon be a login page!")
}

// Endpoint in which to register the users
#[post("/register")]
async fn register_post(
    body: web::Json<RegisterUserSchema>,
    data: web::Data<AppState>,
) -> HttpResponse {
    // Check if the user already exists
    match user_exists(body.username.to_string(), body.email.to_string(), &data).await {
        AuthError::InvalidUsernameError => {
            return HttpResponse::Conflict().json(json!(ErrorResponse::UsernameExists()))
        }
        AuthError::NoError => {
            let _ = register_user(body, data).await;
            return HttpResponse::Conflict().json(json!(ErrorResponse::NoError()));
        }
        _ => return HttpResponse::Conflict().json(json!(ErrorResponse::InternalServerError())),
    }
}

#[get("/register")]
pub async fn register() -> HttpResponse {
    HttpResponse::Ok().body("This will soon be a registration page!")
}

#[get("/users/all")]
pub async fn allusers(data: web::Data<AppState>) -> HttpResponse {
    let users: Vec<UserModel> = sqlx::query_as!(
        UserModel,
        r#"SELECT 
            user_id,
            name,
            role_id, 
            username, 
            email, 
            created_at, 
            dob, 
            profile_id, 
            password 
        FROM 
            USERS
        ORDER BY
            user_ID
        ;"#
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    let user_responses = users
        .into_iter()
        .map(|user| make_user_model_response(&user))
        .collect::<Vec<UserModelResponse>>();

    let json_response = serde_json::json!({
        "status": "success",
        "results": user_responses.len(),
        "users": user_responses
    });
    HttpResponse::Ok().json(json_response)
}

#[get("/logout")]
pub async fn logout(session: Session) -> impl Responder {
    let user: Option<SessionValue> = session.get(&"user").unwrap();
    println!("user: {:?}", user);
    if let Some(x) = user {
        session.purge();
        println!("user {:?} logged out", x);
    } else {
        println!("user not logged in");
    }
    HttpResponse::Ok().json(json!({"status": "success"}))
}

// {"status":"success","token":"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI0IiwiaWF0IjoxNjg1Mjg5MzY4LCJleHAiOjE2ODUyOTI5Njh9.ZurLLa3kxD8EqkyJ6ZHBGlP3-5tLyIu_BcCxpLRaM8A"}⏎

#[get("/users/me")]
async fn get_me_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    _: middleware::JwtMiddleware,
) -> impl Responder {
    let ext = req.extensions();
    let user_id = ext.get::<i32>().unwrap();
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
    user_id = ?",
        user_id.to_string()
    )
    .fetch_one(&data.db)
    .await
    .unwrap();
    let json_response = serde_json::json!({
        "status":  "success",
        "data": serde_json::json!({
            "user": make_user_model_response(&user)
        })
    });

    HttpResponse::Ok().json(json_response)
}
