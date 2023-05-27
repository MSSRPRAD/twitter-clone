use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct ProfileModel {
    profile_id: usize, 
    phone_no: String,
    location: String,
    languages: String,
    user_id: usize,
}