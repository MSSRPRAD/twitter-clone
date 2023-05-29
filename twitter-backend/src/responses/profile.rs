use serde_derive::{Deserialize, Serialize};
use crate::schema::profile::ProfileModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProfileModelResponse {
    pub profile_id: i32, 
    pub phone_no: String,
    pub location: String,
    pub languages: String,
    pub about: String,
}

pub fn make_profile_model_response(profile: &ProfileModel) -> ProfileModelResponse {
    ProfileModelResponse {
        profile_id: profile.profile_id.to_owned(), 
        phone_no: profile.phone_no.to_owned().unwrap(),
        about: profile.about.to_owned().unwrap(),
        location: profile.location.to_owned().unwrap(),
        languages: profile.languages.to_owned().unwrap(),
    }
}