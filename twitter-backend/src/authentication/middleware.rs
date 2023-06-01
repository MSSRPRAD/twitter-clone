use crate::config::AppState;
use crate::errors::auth::AuthError;
use crate::schema::user::{LoginUserSchema, RegisterUserSchema, UserModel};

use actix_web::web;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::PasswordVerifier;
use argon2::{Argon2, PasswordHash, PasswordHasher};

use serde::Serialize;
use serde_derive::Deserialize;

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
        name,
        role_id, 
        username, 
        email, 
        created_at, 
        dob, 
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
        name,
        role_id, 
        username, 
        email, 
        created_at, 
        dob, 
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
