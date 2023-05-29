use serde_derive::{Deserialize, Serialize};
use crate::schema::profile::ProfileModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProfileModelResponse {
    pub profile_id: usize, 
    pub phone_no: String,
    pub location: String,
    pub languages: String,
    pub user_id: usize,
    pub about: String,
}

pub fn make_profile_model_response(profile: &ProfileModel) -> ProfileModelResponse {
    ProfileModelResponse {
        profile_id: profile.profile_id.to_owned(), 
        phone_no: profile.phone_no.to_owned(),
        about: profile.about.to_owned(),
        location: profile.location.to_owned(),
        languages: profile.languages.to_owned(),
        user_id: profile.user_id.to_owned(),
    }
}