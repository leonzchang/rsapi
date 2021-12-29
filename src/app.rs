use actix_rt::System;
use service::create_service;

pub fn run() -> Result<(), std::io::Error> {
    let system = System::new();
    system.block_on(create_service())
}

pub mod service {
    use crate::api;
    use actix_web::{http::header, web, App, HttpResponse, HttpServer};

    pub async fn create_service() -> std::io::Result<()> {
        let static_service_info: &str = Box::leak(service_info().into_boxed_str());

        HttpServer::new(move || {
            App::new()
                .route(
                    "/healthz",
                    web::get().to(move || {
                        HttpResponse::Ok()
                            .insert_header(header::ContentType::json())
                            .body(static_service_info)
                    }),
                )
                .configure(api::routes())
        })
        .bind(("127.0.0.1", 1234))?
        .run()
        .await
    }

    fn service_info() -> String {
        serde_json::json!({
            "name":    env!("CARGO_PKG_NAME"),
            "version": env!("CARGO_PKG_VERSION"),
            "branch": env!("VERGEN_GIT_BRANCH"),
            "commit": env!("VERGEN_GIT_SHA_SHORT"),
            "rustc_host": env!("VERGEN_RUSTC_HOST_TRIPLE"),
            "rustc_semver": env!("VERGEN_RUSTC_SEMVER"),
            "startTime": chrono::Utc::now().timestamp(),
        })
        .to_string()
    }
}
