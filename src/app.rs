use actix_rt::System;
use service::create_service;

pub fn run() -> Result<(), std::io::Error> {
    let system = System::new();
    system.block_on(create_service())
}

pub mod service {
    use crate::api;
    use actix_web::{web, App, HttpResponse, HttpServer};

    pub async fn create_service() -> std::io::Result<()> {
        HttpServer::new(move || {
            App::new()
                .route("/", web::get().to(|| HttpResponse::Ok()))
                .configure(api::routes())
        })
        .bind(("127.0.0.1", 1234))?
        .run()
        .await
    }
}
