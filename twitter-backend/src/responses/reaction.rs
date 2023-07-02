use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ReactionModelResponse {
    pub tweet_id: i32,
    pub reaction_id: i32,
}
