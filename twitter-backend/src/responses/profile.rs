use crate::schema::{profile::ProfileModel, user::UserModel};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ProfileModelResponse {
    pub username: String,
    pub phone_no: String,
    pub location: String,
    pub languages: String,
    pub about: String,
    pub profilepicurl: Option<String>,
    pub bannerurl: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserDetailsResponse {
    pub username: String,
    pub name: String,
    pub dob: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub phone_no: Option<String>,
    pub location: Option<String>,
    pub languages: Option<String>,
    pub about: Option<String>,
    pub profilepicurl: Option<String>,
    pub bannerurl: Option<String>,
}

pub fn make_profile_model_response(profile: &ProfileModel) -> ProfileModelResponse {
    ProfileModelResponse {
        username: profile.username.to_owned(),
        phone_no: profile.phone_no.to_owned().unwrap(),
        about: profile.about.to_owned().unwrap(),
        location: profile.location.to_owned().unwrap(),
        languages: profile.languages.to_owned().unwrap(),
        profilepicurl: profile.profilepicurl.to_owned(),
        bannerurl: profile.bannerurl.to_owned(),
    }
}

pub fn make_user_details_response(profile: &ProfileModel, user: &UserModel) -> UserDetailsResponse {
    UserDetailsResponse {
        username: profile.username.to_owned(),
        name: user.name.to_owned(),
        dob: user.dob.to_owned(),
        created_at: user.created_at.to_owned(),
        phone_no: profile.phone_no.to_owned(),
        location: profile.location.to_owned(),
        languages: profile.languages.to_owned(),
        about: profile.about.to_owned(),
        profilepicurl: profile.profilepicurl.to_owned(),
        bannerurl: profile.bannerurl.to_owned(),
    }
}
