use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ProfileModelResponse {
    profile_id: usize, 
    phone_no: String,
    location: String,
    languages: String,
    user_id: usize,
}