use actix_web::{get, web, HttpRequest, HttpResponse};


use crate::{
    config::AppState,
    functions::{tweet::timeline_for_user, user::get_all_users}, schema::user::UserModel, responses::user::{make_user_model_response, UserModelResponse},
};

#[get("/test/{username}")]
pub async fn test_route(req: HttpRequest, data: web::Data<AppState>) -> HttpResponse {
    let temp = req.uri().to_string();
    let username = temp.split("/").last().unwrap();
    // println!("username: {:?}", username);
    let timeline_tweets = timeline_for_user(username.to_string(), &data).await;
    println!("timeline_tweets: {:?}", timeline_tweets);
    HttpResponse::Ok().body(format!("This is a test page: {:?}", "nothing"))
}


#[get("/users/all")]
pub async fn allusers(data: web::Data<AppState>) -> HttpResponse {
    let users: Vec<UserModel> = get_all_users(&data).await;

    let user_responses = users
        .into_iter()
        .map(|user| make_user_model_response(&user))
        .collect::<Vec<UserModelResponse>>();

    let json_response = serde_json::json!({
        "status": "success",
        "results": user_responses.len(),
        "users": user_responses
    });
    HttpResponse::Ok().json(json_response)
}