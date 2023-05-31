use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RoleModelResponse {
    pub role_id: usize,
}
