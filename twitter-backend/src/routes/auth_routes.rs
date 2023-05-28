use actix_web::{
    get, 
    HttpResponse, 
    web, 
    post, 
    Responder,
    cookie::{time::Duration as ActixWebDuration, Cookie}, 

};
use crate::authentication::middleware::TokenClaims;
use crate::authentication::middleware::JwtMiddleware;
use sqlx::Row;
use crate::schema::user::{UserModel, LoginUserSchema, RegisterUserSchema};
use serde_json::json;
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use argon2::{
    Argon2, 
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString}
};
use crate::{
    responses::user::{make_user_response, UserModelResponse},
    config::AppState,
};

#[get("/login")]
pub async fn login() -> HttpResponse {
    HttpResponse::Ok().body("This will soon be a login page!")
}

#[post("/register")]
async fn register_post(
    body: web::Json<RegisterUserSchema>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let mut transaction = data.db.begin().await;
    println!("reached /register handler");
    let exists: bool = sqlx::query("SELECT EXISTS(SELECT 1 FROM USERS WHERE EMAIL = ?)")
        .bind(body.EMAIL.to_owned())
        .fetch_one(&data.db)
        .await
        .unwrap()
        .get(0);

    if exists {
        println!("User with that email already exists");
        return HttpResponse::Conflict().json(
            serde_json::json!({"status": "fail","message": "User with that email already exists"}),
        );
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.PASSWORD.as_bytes(), &salt)
        .expect("Error while hashing password")
        .to_string();
    println!("What backend recieved: {:?}", body);
    // {"ROLE_ID": 0, "EMAIL": "admin@admin.com","USERNAME": "Admin","PASSWORD": "password123","DOB": "13/01/2003"}
    // Reached here; user does not exist
    let _insert_result = sqlx::query_as!(
        UserModel,
        "INSERT INTO USERS 
            (ROLE_ID, USERNAME, PASSWORD, EMAIL, DOB) 
        VALUES 
            (?, ?, ?, ?, ?);",
        0,
        body.USERNAME.to_string(),
        body.EMAIL.to_string().to_lowercase(),
        hashed_password,
        body.DOB.to_string(),
    )
    .execute(&data.db)
    .await;
    transaction.unwrap().commit().await;
    println!("USER INSERTED!");
    let query_result = sqlx::query_as!(UserModel, 
        "
        SELECT 
            USER_ID, 
            ROLE_ID, 
            USERNAME, 
            EMAIL, 
            CREATED_AT, 
            DOB, 
            PROFILE_ID, 
            PASSWORD
        FROM 
            USERS
        WHERE 
            (ROLE_ID, USERNAME, EMAIL, PASSWORD, DOB) = (?, ?, ?, ?, ?);", 
            0,
            body.USERNAME.to_string(),
            body.EMAIL.to_string().to_lowercase(),
            hashed_password,
            body.DOB.to_string()
        )
        .fetch_one(&data.db)
        .await;
    println!("USER Queried from db: {:?}", query_result);
    match query_result {
        Ok(user) => {
            let user_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "user": make_user_response(&user)
            })});
            return HttpResponse::Ok().json(user_response);
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    }
    
}

#[post("/login")]
async fn login_post(
    body: web::Json<LoginUserSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let user = sqlx::query_as!(UserModel, "SELECT * FROM USERS WHERE EMAIL = ?", body.EMAIL)
        .fetch_optional(&data.db)
        .await
        .unwrap()
        .unwrap();

    let parsed_hash = PasswordHash::new(&user.PASSWORD).unwrap();

    let is_valid = Argon2::default()
        .verify_password(body.PASSWORD.as_bytes(), &parsed_hash)
        .map_or(false, |_| true);

    if !is_valid {
        return HttpResponse::BadRequest()
            .json(json!({"status": "fail", "message": "Invalid email or password"}));
    }

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user.USER_ID.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.env.jwt_secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build("token", token.to_owned())
        .path("/")
        .max_age(ActixWebDuration::new(60 * 60, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success", "token": token}))
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
            USER_ID,
            ROLE_ID, 
            USERNAME, 
            EMAIL, 
            CREATED_AT, 
            DOB, 
            PROFILE_ID, 
            PASSWORD 
        FROM 
            USERS
        ORDER BY
            USER_ID
        ;"#
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    let user_responses = users
    .into_iter()
    .map(|user| make_user_response(&user))
    .collect::<Vec<UserModelResponse>>();

    let json_response = serde_json::json!({
        "status": "success",
        "results": user_responses.len(),
        "users": user_responses
    });
    HttpResponse::Ok().json(json_response)
}


#[get("/logout")]
pub async fn logout(_: JwtMiddleware) -> impl Responder {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success"}))
}
