use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct ProfileModel {
    pub profile_id: usize, 
    pub about: String,
    pub phone_no: String,
    pub location: String,
    pub languages: String,
    pub user_id: usize,
}