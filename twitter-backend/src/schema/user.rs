use serde_derive::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct UserModel {
    pub user_id: i32, 
    pub role_id: i32,
    pub username: String, 
    pub name: String,
    pub email: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub dob: String,
    pub profile_id: Option<i32>,
    pub password: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String,
}
#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct RegisterUserSchema {
    pub role_id: i32,
    pub username: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub dob: String,
}