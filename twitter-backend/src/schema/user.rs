use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct UserModel {
    user_id: usize, 
    user_name: String, 
    email: String,
    created_stamp: Option<chrono::DateTime<chrono::Utc>>,
    dob: String,
    profile_id: usize,
    password: String,
}