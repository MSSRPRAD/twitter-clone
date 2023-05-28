use actix_web::{
    get, 
    HttpResponse, 
    web, 
    post, 
    Responder,
    cookie::{time::Duration as ActixWebDuration, Cookie}, 
    HttpRequest, HttpMessage,

};
use crate::authentication::{middleware::{TokenClaims, self}};
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

#[post("/login")]
async fn login_post(
    body: web::Json<LoginUserSchema>,
    data: web::Data<AppState>,
) -> impl Responder {

    // See if the username exists
    let user = sqlx::query_as!(UserModel, "SELECT * FROM USERS WHERE username = ?", body.username)
        .fetch_optional(&data.db)
        .await
        .unwrap()
        .unwrap();
    // If it does, check if the password is correct
    let parsed_hash = PasswordHash::new(&user.password).unwrap();

    let is_valid = Argon2::default()
        .verify_password(body.password.as_bytes(), &parsed_hash)
        .map_or(false, |_| true);
    println!("valid: {}", is_valid);
    // If it is not valid, return a BadRequest response
    if !is_valid {
        return HttpResponse::BadRequest()
            .json(json!({"status": "fail", "message": "Invalid username or password"}));
    }
    // If it is valid, generate a jwt token and return it
    // User will use this token from now onwards in future requests

    // Some Magic for generating jwt tokens
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user.user_id.to_string(),
        exp,
        iat,
    };

    // Generating the token
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.env.jwt_secret.as_ref()),
    )
    .unwrap();

    // Returning the token in a cookie
    let cookie = Cookie::build("token", token.to_owned())
        .path("/")
        .max_age(ActixWebDuration::new(60 * 60, 0))
        .http_only(true)
        .finish();
    // Return token with response
    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success", "token": token}))
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
    let exists: bool = sqlx::query("SELECT EXISTS(SELECT 1 FROM USERS WHERE email = ?)")
        .bind(body.email.to_owned())
        .fetch_one(&data.db)
        .await
        .unwrap()
        .get(0);
    // If the user already exists, return a Conflict response
    if exists {
        return HttpResponse::Conflict().json(
            serde_json::json!({"status": "fail","message": "User with that email already exists"}),
        );
    }
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
    // Query the db and return a response containing the newly added user tuple
    // This confirms that the operation succeeded
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
    // If the query fails, return an InternalServerError response
    // Otherwise, return the newly added user tuple
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

// {"status":"success","token":"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI0IiwiaWF0IjoxNjg1Mjg5MzY4LCJleHAiOjE2ODUyOTI5Njh9.ZurLLa3kxD8EqkyJ6ZHBGlP3-5tLyIu_BcCxpLRaM8A"}⏎


#[get("/users/me")]
async fn get_me_handler(
    req: HttpRequest,
    data: web::Data<AppState>,
    _: middleware::JwtMiddleware,
) -> impl Responder {

    println!("reached /users/me!");
    let ext = req.extensions();
    let user_id = ext.get::<i32>().unwrap();
    println!("user_id: {:?}", user_id);
    let user = sqlx::query_as!(UserModel, "
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
        .await
        .unwrap();
    let json_response = serde_json::json!({
        "status":  "success",
        "data": serde_json::json!({
            "user": make_user_response(&user)
        })
    });
    println!("json response generated: {}", json_response);

    HttpResponse::Ok().json(json_response)
}