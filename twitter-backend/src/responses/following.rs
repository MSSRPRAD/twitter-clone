use serde_derive::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct FollowingModelResponse {
    pub username: String,
    pub following: String
}