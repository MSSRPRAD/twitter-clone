use std::future::{ready, Ready};
use crate::config::AppState;
use crate::schema::user::{LoginUserSchema, UserModel};
use actix_web::error::ErrorUnauthorized;
use actix_web::{dev::Payload, Error as ActixWebError};
use actix_web::{http, web, FromRequest, HttpMessage, HttpRequest, App};
use argon2::{PasswordHash, Argon2};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Serialize;
use crate::authentication::errors::ErrorResponse;
use serde_derive::Deserialize;
use crate::errors::auth::AuthError;
use argon2::PasswordVerifier;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

pub struct JwtMiddleware {
    pub user_id: i32,
}

pub async fn user_exists(user_id: i32, data: web::Data<AppState>) -> bool {
    let option_user = sqlx::query_as!(UserModel, "
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
    user_id = ?", user_id.to_string())
        .fetch_one(&data.db)
        .await;
    match option_user {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub async fn validate_credentials(loginuser: &LoginUserSchema, data: web::Data<AppState>) -> AuthError {
    let option_user = sqlx::query_as!(UserModel, "
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
    username = ?", loginuser.username.to_string())
        .fetch_one(&data.db)
        .await;
    match option_user {
        Ok(_) => {
            let user = option_user.unwrap();
            // If it does, check if the password is correct
            let parsed_hash = PasswordHash::new(&user.password).unwrap();

            let is_valid = Argon2::default()
                .verify_password(user.password.as_bytes(), &parsed_hash)
                .map_or(false, |_| true);
            println!("valid: {}", is_valid);
            // If it is not valid, return a BadRequest response
            if !is_valid {
               return AuthError::WrongPasswordError;
            } else {
                return AuthError::NoError;
            }
        },
        Err(_) => {
            return AuthError::InvalidUsernameError;
        },
    }
}

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
        let user_id : i32 = claims.sub.as_str().to_owned().parse().unwrap();
        println!("user_id: {}", user_id);
        req.extensions_mut()
            .insert::<i32>(user_id.to_owned());

        ready(Ok(JwtMiddleware { user_id }))
    }
}