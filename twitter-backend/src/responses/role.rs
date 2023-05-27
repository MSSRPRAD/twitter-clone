use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RoleModelResponse {
    pub role_id: usize,
}