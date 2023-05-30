use serde_derive::{Serialize, Deserialize};

pub enum AuthError {
    InvalidUsernameError,
    WrongPasswordError,
    NoError,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

impl ErrorResponse {
    pub fn InvalidCredentials() -> Self {
        Self {
            status: "fail".to_string(),
            message: "Invalid Credentials".to_string(),
        }
    }
    pub fn InvalidUser() -> Self {
        Self {
            status: "fail".to_string(),
            message: "User doesn't exist".to_string(),
        }
    }
    pub fn NotLoggedIn() -> Self {
        Self { 
            status: "fail".to_string(), 
            message: "You are not logged in, please provide token".to_string(),
        }
    }
    pub fn NoError() -> Self {
        Self { 
            status: "success".to_string(), 
            message: "Authentication Succeeded".to_string(),
        }
    }
}
