use crate::schema::user::UserModel;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserModelResponse {
    pub role_id: i32,
    pub user_id: i32,
    pub name: String,
    pub username: String,
    pub email: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub dob: String,
    pub profile_id: Option<i32>,
    pub password: String,
}

pub fn make_user_model_response(user: &UserModel) -> UserModelResponse {
    UserModelResponse {
        role_id: user.role_id.to_owned(),
        user_id: user.user_id.to_owned(),
        name: user.name.to_owned(),
        username: user.username.to_owned(),
        email: user.email.to_owned(),
        created_at: user.created_at.to_owned(),
        dob: user.dob.to_owned(),
        profile_id: user.profile_id.to_owned(),
        password: user.password.to_owned(),
    }
}
