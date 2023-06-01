use actix_web::web;

use crate::{schema::user::UserModel, config::AppState};

pub async fn user_from_username(username: String, data: &web::Data<AppState>) -> Option<UserModel> {
    let option_user = sqlx::query_as!(
    UserModel,
    "SELECT role_id, username, name, password, email, created_at, dob
    FROM USERS
    WHERE 
    username = ?;",
        username
    )
    .fetch_one(&data.db)
    .await;

    match option_user.as_ref() {
        Ok(_) => {
            return Option::from(option_user.unwrap());
        }
        Err(_) => {
            return None;
        }
    }
}