use crate::config::AppState;
use crate::responses::following::FollowingModelResponse;
use crate::schema::following::{number, FollowingModel};
use crate::{errors::auth::AuthError, responses::following::FollowingDetailsResponse};
use actix_web::web;

pub async fn create_or_update_following(
    body: FollowingModelResponse,
    data: web::Data<AppState>,
) -> AuthError {
    let _delete_result = sqlx::query_as!(
        FollowingModel,
        "DELETE FROM FOLLOWING WHERE username = ? AND following = ?;",
        body.username,
        body.following,
    )
    .execute(&data.db)
    .await;
    let _insert_result = sqlx::query_as!(
        FollowingModel,
        "INSERT INTO FOLLOWING
            (username, following) 
        VALUES 
            (?, ?);",
        body.username,
        body.following,
    )
    .execute(&data.db)
    .await;
    return AuthError::NoError;
}

pub async fn get_following_details_response(
    requested_username: &str,
    requesting_username: &str,
    data: web::Data<AppState>,
) -> FollowingDetailsResponse {
    let mut details: FollowingDetailsResponse = FollowingDetailsResponse {
        requesting: requesting_username.to_string(),
        requested: requested_username.to_string(),
        following: true,
        is_followed: true,
        no_of_followers: 0,
        no_of_following: 0,
    };

    let option_requesting_follows_requested = sqlx::query_as!(
        FollowingModel,
        "
    SELECT
        username,
        following, 
        created_at
    FROM FOLLOWING
    WHERE 
    username = ? AND following = ?",
        requesting_username,
        requested_username,
    )
    .fetch_one(&data.db)
    .await;
    match option_requesting_follows_requested {
        Ok(_) => {
            // If it does, do nothing
        }
        Err(_) => {
            details.following = false;
        }
    }

    let option_requested_follows_requesting = sqlx::query_as!(
        FollowingModel,
        "
    SELECT
        username,
        following, 
        created_at
    FROM FOLLOWING
    WHERE 
    username = ? AND following = ?",
        requested_username,
        requesting_username,
    )
    .fetch_one(&data.db)
    .await;
    match option_requested_follows_requesting {
        Ok(_) => {
            // If it does, do nothing
        }
        Err(_) => {
            details.is_followed = false;
        }
    }

    let no_following = sqlx::query_as!(
        number,
        "
    SELECT COUNT(*) as number
    FROM FOLLOWING
    WHERE
    username = ?",
        requesting_username,
    )
    .fetch_one(&data.db)
    .await;
    match option_requested_follows_requesting {
        Ok(_) => {
            // If it does, do nothing
            details.no_of_following = no_following.unwrap().number as i32;
        }
        Err(_) => {
            // details.is_followed = false;
        }
    }

    let no_following = sqlx::query_as!(
        number,
        "
    SELECT COUNT(*) as number
    FROM FOLLOWING
    WHERE
    following = ?",
        requesting_username,
    )
    .fetch_one(&data.db)
    .await;
    match option_requested_follows_requesting {
        Ok(_) => {
            // If it does, do nothing
            details.no_of_followers = no_following.unwrap().number as i32;
        }
        Err(_) => {
            // details.is_followed = false;
        }
    }

    return details;
}
