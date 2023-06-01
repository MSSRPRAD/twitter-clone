use serde_derive::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct UserModel {
    pub role_id: i32,
    pub username: String,
    pub name: String,
    pub email: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub dob: String,
    pub password: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    pub username: String,
    pub password: String,
    pub role_id: i32,
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
