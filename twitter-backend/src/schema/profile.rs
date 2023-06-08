use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct ProfileModel {
    pub username: String,
    pub about: Option<String>,
    pub phone_no: Option<String>,
    pub location: Option<String>,
    pub languages: Option<String>,
    pub profilepicurl: Option<String>,
    pub bannerurl: Option<String>,
}
