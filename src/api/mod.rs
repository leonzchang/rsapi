use actix_web::{get, web, Error, HttpResponse};

pub fn routes() -> impl FnOnce(&mut web::ServiceConfig) {
    move |config: &mut web::ServiceConfig| {
        config.service(web::scope("api").service(handle_test).service(handle_home));
    }
}

//  TODO: Err handling, SQL query, database setup

#[get("/test")]
pub async fn handle_test() -> Result<HttpResponse, Error> {
    let result = serde_json::json!({ "test": "Hello from api!" });
    Ok(HttpResponse::Ok().json(result))
}

#[get("/home/{username}")]
pub async fn handle_home(query: web::Path<String>) -> Result<HttpResponse, Error> {
    let username = query.into_inner();
    let result = serde_json::json!({ "home": String::from("Hello ")+username.as_str()+"!" });
    Ok(HttpResponse::Ok().json(result))
}
