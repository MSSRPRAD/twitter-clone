use actix_web::web;

use crate::{responses::tweet::CreateTweetModelResponse, config::AppState, errors::auth::AuthError};

pub async fn create_tweet(
    body: web::Json<CreateTweetModelResponse>,
    data: web::Data<AppState>,
    username: String,
) -> AuthError {
    // Insert the user into the database
    let _insert_result = sqlx::query_as!(
        TweetModel,
        "INSERT INTO TWEETS (username, parent_id, content, quote_id)
        VALUES 
            (?, ?, ?, ?);",
        username,
        body.parent_id,
        body.content,
        body.quote_id,
    )
    .execute(&data.db)
    .await;
    return AuthError::NoError;
}