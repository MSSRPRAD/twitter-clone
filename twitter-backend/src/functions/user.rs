use actix_web::web;

use crate::{config::AppState, schema::user::UserModel};

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

pub async fn get_all_users(data: &web::Data<AppState>) -> Vec<UserModel> {
    let all_users = sqlx::query_as!(
        UserModel,
        r#"SELECT 
            name,
            role_id, 
            username, 
            email, 
            created_at, 
            dob, 
            password 
        FROM 
            USERS
        ORDER BY
            created_at
        ;"#
    )
    .fetch_all(&data.db)
    .await
    .unwrap_or(Vec::new());

    return all_users
}
