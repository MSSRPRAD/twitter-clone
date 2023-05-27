use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct UserModel {
    pub USER_ID: i32, 
    pub USERNAME: String, 
    pub EMAIL: String,
    pub CREATED_AT: Option<chrono::DateTime<chrono::Utc>>,
    pub DOB: String,
    pub PROFILE_ID: Option<i32>,
    pub PASSWORD: String,
}