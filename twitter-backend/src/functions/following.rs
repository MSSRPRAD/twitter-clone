use crate::errors::auth::AuthError;
use crate::responses::following::FollowingModelResponse;
use crate::schema::following::FollowingModel;
use crate::config::AppState;
use actix_web::{web};

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