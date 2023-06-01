use crate::schema::profile::ProfileModel;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ProfileModelResponse {
    pub username: String,
    pub phone_no: String,
    pub location: String,
    pub languages: String,
    pub about: String,
}

pub fn make_profile_model_response(profile: &ProfileModel) -> ProfileModelResponse {
    ProfileModelResponse {
        username: profile.username.to_owned(),
        phone_no: profile.phone_no.to_owned().unwrap(),
        about: profile.about.to_owned().unwrap(),
        location: profile.location.to_owned().unwrap(),
        languages: profile.languages.to_owned().unwrap(),
    }
}
