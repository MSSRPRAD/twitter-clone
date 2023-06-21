use serde_derive::{Deserialize, Serialize};
pub enum AuthError {
    InvalidUsernameError,
    WrongPasswordError,
    NoError,
    UserExistsError,
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
    pub fn NoProfile() -> Self {
        Self {
            status: "fail".to_string(),
            message: "No Profile for this user.".to_string(),
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
    pub fn UsernameExists() -> Self {
        Self {
            status: "success".to_string(),
            message: "Username Exists".to_string(),
        }
    }
    pub fn InternalServerError() -> Self {
        Self {
            status: "success".to_string(),
            message: "Internal Server Error".to_string(),
        }
    }
}
