use serde_derive::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct FollowingModelResponse {
    pub username: String,
    pub following: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct FollowingDetailsResponse {
    pub requesting: String,
    pub requested: String,
    pub following: bool,
    pub is_followed: bool,
    pub no_of_followers: i32,
    pub no_of_following: i32,
}
