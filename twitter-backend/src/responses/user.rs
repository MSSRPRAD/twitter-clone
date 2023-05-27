use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserModelResponse {
    user_id: usize, 
    user_name: String, 
    email: String,
    created_stamp: Option<chrono::DateTime<chrono::Utc>>,
    dob_id: usize,
    profile_id: usize,
    password: String,
}