use crate::table::SqlSource;
use crate::utils::actix_ext::SafeData;
use crate::{app::service, table};
use actix_web::{error::ResponseError, get, http::StatusCode, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("table error: `{0}`")]
    CoreErr(#[from] table::Error),
}

impl Error {
    fn to_codes(&self) -> (i16, StatusCode, Option<String>) {
        match self {
            _ => (0, StatusCode::INTERNAL_SERVER_ERROR, None),
        }
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        let (code, status_code, context) = self.to_codes();
        HttpResponse::build(status_code).json(ErrResponse {
            error: ErrInner { code, context },
        })
    }
}
#[derive(Serialize)]
struct ErrInner {
    code: i16,
    context: Option<String>,
}

#[derive(Serialize)]
struct ErrResponse {
    error: ErrInner,
}

pub fn routes() -> impl FnOnce(&mut web::ServiceConfig) {
    move |config: &mut web::ServiceConfig| {
        config.service(
            web::scope("api")
                .service(handle_test)
                .service(handle_home)
                .service(handle_get_persons)
                .service(handle_post_person_test),
        );
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

#[get("/person/{name}")]
pub async fn handle_post_person_test(
    pool: SafeData<PgPool>,
    query: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let name = query.into_inner();

    let id = pool.update_data(name).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "person_id": id })))
}

#[get("/persons")]
pub async fn handle_get_persons(conns: SafeData<PgPool>) -> Result<HttpResponse, Error> {
    #[derive(Serialize)]
    struct DataResponse {
        person_id: i32,
        person_name: String,
    }

    let result = conns
        .get_all_data()
        .await?
        .into_iter()
        .map(|(person_id, person_name)| DataResponse {
            person_id,
            person_name,
        })
        .collect::<Vec<DataResponse>>();

    Ok(HttpResponse::Ok().json(result))
}

#[derive(Serialize, Deserialize)]
struct PersonData {
    person_name: String,
}
//TODO post method
#[post("/person")]
pub async fn handle_post_person(
    pool: SafeData<PgPool>,
    data: web::Json<PersonData>,
) -> Result<HttpResponse, Error> {
    let name = query.into_inner();

    let id = pool.post_data(name).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({ "person_id": id })))
}
