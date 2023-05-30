use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct ProfileModel {
    pub profile_id: i32, 
    pub about: Option<String>,
    pub phone_no: Option<String>,
    pub location: Option<String>,
    pub languages: Option<String>,
}





