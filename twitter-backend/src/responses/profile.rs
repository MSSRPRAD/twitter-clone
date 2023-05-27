use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ProfileModelResponse {
    profile_id: usize, 
    phone_no: String, 
    address: String, 
    location: String,
    languages: Vec<String>,
    user_id: usize,
}