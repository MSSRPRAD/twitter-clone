use crate::config::AppState;

use actix_web::{get, web, HttpRequest, HttpResponse};
use serde_json::Value;

#[get("/profile/{username}")]
pub async fn profile_username(
    _req: HttpRequest,
    _data: web::Data<AppState>,
    // _: middleware::JwtMiddleware,
) -> HttpResponse {
    let json_response: Value = Default::default();
    // let username = req.query_string().split("/").last().unwrap();
    // println!("username: {:?}", username);
    // // match user_exists(username, email, &data) {
    // //     todo!();
    // // }
    // // match profile_from_username(username.to_string(), &data).await {
    // //     None => {
    // //         json_response = serde_json::json!(ErrorResponse::);
    // //         return HttpResponse::NotFound().body("Profile not found!");
    // //     }
    // // }
    // let json_response = serde_json::json!({
    //     "status":  "success",
    //     "data": {
    //         "profile": serde_json::json!({
    //             "profile": make_profile_model_response(&profile)
    //         }),
    //         "user": serde_json::json!({
    //             "user": make_user_model_response(&user)
    //         })
    //     }
    // });
    HttpResponse::Ok().json(json_response)
}

#[get("/twitter/{username}")]
pub async fn tweets() -> HttpResponse {
    HttpResponse::Ok().body("This will soon be the tweets page!")
}

#[get("/twitter/{username}/with_replies")]
pub async fn tweets_with_replies() -> HttpResponse {
    HttpResponse::Ok().body("This will soon be the tweets with replies page!")
}

#[get("/twitter/{username}/images")]
pub async fn tweets_images() -> HttpResponse {
    HttpResponse::Ok().body("This will soon be the image tweets page!")
}

#[get("/twitter/{username}/likes")]
pub async fn tweets_likes() -> HttpResponse {
    HttpResponse::Ok().body("This will soon be the liked tweets page!")
}
