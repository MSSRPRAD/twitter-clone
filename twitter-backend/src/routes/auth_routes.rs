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
    println!("reached /register handler");
    let exists: bool = sqlx::query("SELECT EXISTS(SELECT 1 FROM USERS WHERE email = ?)")
        .bind(body.email.to_owned())
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
        .hash_password(body.password.as_bytes(), &salt)
        .expect("Error while hashing password")
        .to_string();
    println!("What backend recieved: {:?}", body);
    // {"role_id": 0, "email": "admin@admin.com","username": "Admin","password": "password123","dob": "13/01/2003"}
    // Reached here; user does not exist
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
    println!("user INSERTED!");
    let query_result = sqlx::query_as!(UserModel, 
        "
        SELECT 
            user_id, 
            role_id, 
            username,
            name, 
            email, 
            created_at, 
            dob, 
            profile_id, 
            password
        FROM 
            USERS
        WHERE 
            (role_id, username, email, password, dob) = (?, ?, ?, ?, ?);", 
            body.role_id,
            body.username.to_string(),
            body.email.to_string().to_lowercase(),
            hashed_password,
            body.dob.to_string()
        )
        .fetch_one(&data.db)
        .await;
    println!("user Queried from db: {:?}", query_result);
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
    let user = sqlx::query_as!(UserModel, "SELECT * FROM USERS WHERE email = ?", body.email)
        .fetch_optional(&data.db)
        .await
        .unwrap()
        .unwrap();

    let parsed_hash = PasswordHash::new(&user.password).unwrap();

    let is_valid = Argon2::default()
        .verify_password(body.password.as_bytes(), &parsed_hash)
        .map_or(false, |_| true);

    if !is_valid {
        return HttpResponse::BadRequest()
            .json(json!({"status": "fail", "message": "Invalid email or password"}));
    }

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user.user_id.to_string(),
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
