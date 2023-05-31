use crate::authentication::errors::ErrorResponse;
use crate::config::AppState;
use crate::errors::auth::AuthError;
use crate::schema::user::{LoginUserSchema, RegisterUserSchema, UserModel};
use actix_web::cookie::time::error;
use actix_web::cookie::Cookie;
use actix_web::dev::ServiceResponse;
use actix_web::error::ErrorInternalServerError;
use actix_web::error::ErrorUnauthorized;
use actix_web::http::header::HeaderValue;
use actix_web::http::{header, Error};
use actix_web::web::Data;
use actix_web::{dev::Payload, Error as ActixWebError};
use actix_web::{http, web, FromRequest, HttpMessage, HttpRequest};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::PasswordVerifier;
use argon2::{Argon2, PasswordHash, PasswordHasher};
use deadpool_redis::redis::AsyncCommands;
use deadpool_redis::{Connection, Manager, Pool};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Serialize;
use serde_derive::Deserialize;
use std::future::{ready, Ready};
use time::{self, Duration};

// pub async fn get_redis_con(sessiondb: Data<Pool>) -> Connection {
//     return sessiondb
//         .get()
//         .await
//         .map_err(|e| {
//             actix_web::HttpResponse::InternalServerError().json(crate::errors::auth::ErrorResponse{
//                 status: "failed".to_string(),
//                 message: "could not connect to redis".to_string(),
//             })
//         })
//         .expect("Redis connection cannot be gotten.");
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionValue {
    pub username: String,
    pub role_id: i32,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

pub struct JwtMiddleware {
    pub user_id: i32,
}

pub async fn register_user(
    body: web::Json<RegisterUserSchema>,
    data: web::Data<AppState>,
) -> AuthError {
    // Some magic for hashing the password
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .expect("Error while hashing password")
        .to_string();
    // Insert the user into the database
    let _insert_result = sqlx::query_as!(
        UserModel,
        "INSERT INTO USERS 
            (role_id, name, username, password, email, dob) 
        VALUES 
            (?, ?, ?, ?, ?, ?);",
        body.role_id,
        body.name.to_string(),
        body.username.to_string(),
        hashed_password,
        body.email.to_string().to_lowercase(),
        body.dob.to_string(),
    )
    .execute(&data.db)
    .await;
    return AuthError::NoError;
}

pub async fn user_exists(username: String, email: String, data: &web::Data<AppState>) -> AuthError {
    let option_user = sqlx::query_as!(
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
    username = ? AND email = ?",
        username,
        email
    )
    .fetch_one(&data.db)
    .await;
    match option_user {
        Ok(_) => {
            // If it does, return an error
            return AuthError::UserExistsError;
        }
        Err(_) => {
            return AuthError::NoError;
        }
    }
}

pub async fn validate_credentials(
    loginuser: &LoginUserSchema,
    data: web::Data<AppState>,
) -> AuthError {
    let option_user = sqlx::query_as!(
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
    username = ? AND role_id = ?",
        loginuser.username.to_string(),
        loginuser.role_id
    )
    .fetch_one(&data.db)
    .await;
    match option_user {
        Ok(_) => {
            let user = option_user.unwrap();
            // If it does, check if the password is correct
            let parsed_hash = PasswordHash::new(&user.password).unwrap();

            let is_valid = Argon2::default()
                .verify_password(loginuser.password.as_bytes(), &parsed_hash)
                .map_or(false, |_| true);
            // If it is not valid, return a BadRequest response
            if !is_valid {
                return AuthError::WrongPasswordError;
            } else {
                return AuthError::NoError;
            }
        }
        Err(_) => {
            return AuthError::InvalidUsernameError;
        }
    }
}

// /// invalidates session cookie
// fn remove_cookie<B>(&self, res: &mut ServiceResponse<B>) -> Result<(), Error> {
//     let mut cookie = Cookie::named(self.name.clone());
//     cookie.set_value("");
//     cookie.set_max_age(Duration::seconds(0));
//     cookie.set_expires(Duration::ne(&self, other) - Duration::days(365));

//     let val = HeaderValue::from_str(&cookie.to_string())
//         .map_err(ErrorInternalServerError);
//     res.headers_mut().append(header::SET_COOKIE, val);

//     Ok(())
// }

// Pure Magic. Don't touch!!!!
impl FromRequest for JwtMiddleware {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // println!("Inside from_request()");
        let data = req.app_data::<web::Data<AppState>>().unwrap();

        let token = req
            .cookie("token")
            .map(|c| c.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
            });
        // Reached here
        println!("token: {:?}", token);
        if token.is_none() {
            let json_error = ErrorResponse {
                status: "fail".to_string(),
                message: "You are not logged in, please provide token".to_string(),
            };
            return ready(Err(ErrorUnauthorized(json_error)));
        }

        let claims = match decode::<TokenClaims>(
            &token.unwrap(),
            &DecodingKey::from_secret(data.env.jwt_secret.as_ref()),
            &Validation::default(),
        ) {
            Ok(c) => c.claims,
            Err(_) => {
                let json_error = ErrorResponse {
                    status: "fail".to_string(),
                    message: "Invalid token".to_string(),
                };
                return ready(Err(ErrorUnauthorized(json_error)));
            }
        };
        println!("claims: {:?}", claims);
        println!("parsing\n{:?}", uuid::Uuid::parse_str(claims.sub.as_str()));
        let user_id: i32 = claims.sub.as_str().to_owned().parse().unwrap();
        println!("user_id: {}", user_id);
        req.extensions_mut().insert::<i32>(user_id.to_owned());

        ready(Ok(JwtMiddleware { user_id }))
    }
}
