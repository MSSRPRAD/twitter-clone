use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct RoleModel {
    role_id: usize,
}