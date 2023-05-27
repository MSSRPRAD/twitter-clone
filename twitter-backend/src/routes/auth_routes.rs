use actix_web::{get, HttpResponse, web};
use crate::{
    schema::user::UserModel,
    responses::user::{make_user_response, UserModelResponse},
    appstate::AppState,
};

#[get("/login")]
pub async fn login() -> HttpResponse {
    HttpResponse::Ok().body("This will soon be a login page!")
}

#[get("/register")]
pub async fn register() -> HttpResponse {
    HttpResponse::Ok().body("This will soon be a registration page!")
}

#[get("/users")]
pub async fn users(data: web::Data<AppState>) -> HttpResponse {

    let users: Vec<UserModel> = sqlx::query_as!(
        UserModel, 
        r#"SELECT 
            USER_ID, 
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
        "results": user_responses.len()
    });
    HttpResponse::Ok().json(json_response)
}
