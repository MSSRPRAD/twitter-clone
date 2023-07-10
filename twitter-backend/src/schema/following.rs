use serde_derive::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct FollowingModel {
    pub username: String,
    pub following: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct followers {
    pub followers: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct number {
    pub number: i64,
}
