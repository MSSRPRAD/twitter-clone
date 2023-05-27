use serde_derive::{Deserialize, Serialize};
use crate::schema::user::UserModel;
#[derive(Debug, Deserialize, Serialize)]
pub struct UserModelResponse {
    pub USER_ID: i32, 
    pub USERNAME: String, 
    pub EMAIL: String,
    pub CREATED_AT: Option<chrono::DateTime<chrono::Utc>>,
    pub DOB: String,
    pub PROFILE_ID: i32,
    pub PASSWORD: String,
}

pub fn make_user_response(user: &UserModel) -> UserModelResponse {
    UserModelResponse {
        USER_ID: user.USER_ID.to_owned(), 
        USERNAME: user.USERNAME.to_owned(), 
        EMAIL: user.EMAIL.to_owned(),
        CREATED_AT: user.CREATED_AT.to_owned(),
        DOB: user.DOB.to_owned(),
        PROFILE_ID: user.PROFILE_ID.unwrap().to_owned(),
        PASSWORD: user.PASSWORD.to_owned(),
    }
}
