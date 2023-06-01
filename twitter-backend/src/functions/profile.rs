use crate::config::AppState;
use crate::errors::profile::ProfileError;
use crate::responses::profile::ProfileModelResponse;
use crate::schema::profile::ProfileModel;
use actix_web::web;

// pub async fn fetch_user_details(

// )

pub async fn profile_exists(username: String, data: &web::Data<AppState>) -> ProfileError {
    let option_profile = sqlx::query_as!(
        ProfileModel,
        "SELECT PROFILES.username, phone_no, location, languages, about
    FROM PROFILES, USERS
    WHERE 
    PROFILES.username = ?
    AND
    USERS.username = PROFILES.username",
        username
    )
    .fetch_one(&data.db)
    .await;
    match option_profile {
        Ok(_) => {
            // If it does, return an error
            return ProfileError::ProfileExists;
        }
        Err(_) => {
            return ProfileError::NoError;
        }
    }
}

pub async fn create_or_update_profile(
    body: web::Json<ProfileModelResponse>,
    data: web::Data<AppState>,
) -> ProfileError {
    let _delete_result = sqlx::query_as!(
        ProfileModel,
        "DELETE FROM PROFILES WHERE username = ?;",
        body.username,
    )
    .execute(&data.db)
    .await;
    let _insert_result = sqlx::query_as!(
        ProfileModel,
        "INSERT INTO PROFILES
            (username, phone_no, location, languages, about) 
        VALUES 
            (?, ?, ?, ?, ?);",
        body.username,
        body.phone_no,
        body.location,
        body.languages,
        body.about,
    )
    .execute(&data.db)
    .await;
    return ProfileError::NoError;
}

pub async fn profile_from_username(
    username: String,
    data: &web::Data<AppState>,
) -> Option<ProfileModel> {
    let option_profile = sqlx::query_as!(
        ProfileModel,
        "SELECT USERS.username, phone_no, location, languages, about
    FROM PROFILES, USERS
    WHERE 
    USERS.username = ?
    AND
    USERS.username = PROFILES.username",
        username
    )
    .fetch_one(&data.db)
    .await;

    match option_profile.as_ref() {
        Ok(_) => {
            return Option::from(option_profile.unwrap());
        }
        Err(_) => {
            return None;
        }
    }
}
