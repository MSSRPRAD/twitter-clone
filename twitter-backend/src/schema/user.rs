use serde_derive::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct UserModel {
    pub USER_ID: i32, 
    pub ROLE_ID: i32,
    pub USERNAME: String, 
    pub EMAIL: String,
    pub CREATED_AT: Option<chrono::DateTime<chrono::Utc>>,
    pub DOB: String,
    pub PROFILE_ID: Option<i32>,
    pub PASSWORD: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    pub EMAIL: String,
    pub PASSWORD: String,
}
#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct RegisterUserSchema {
    pub USERNAME: String,
    pub EMAIL: String,
    pub PASSWORD: String,
    pub DOB: String,
}