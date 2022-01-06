use actix_rt::System;
use service::{create_service, init_connections};
use structopt::StructOpt;
#[derive(Debug, StructOpt)]
pub struct Opts {
    /// Postgres database url in pg format
    #[structopt(
        name = "pg",
        default_value = "postgres://postgres:test123@:5432/domain?sslmode=disable",
        env = "ORACLE_POSTGRES"
    )]
    pub database_url: String,
}

pub fn run(opts: Opts) -> Result<(), std::io::Error> {
    env_logger::init();

    let Opts { database_url } = opts;
    let system = System::new();
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("tokio runtime needed to continue. qed");
    let pg_pool = rt.block_on(async move { init_connections(&database_url).await });

    system.block_on(create_service(pg_pool))
}

pub mod service {
    use crate::api;
    use crate::utils::actix_ext::SafeData;
    use actix_web::{http::header, web, App, HttpResponse, HttpServer};
    use sqlx::postgres::PgPoolOptions;
    use sqlx::PgPool;

    pub async fn init_connections(database_url: &str) -> PgPool {
        // Create a connection pool
        // can set the acquire timeout when acquiring a connection
        // #NOTE unit tests and examples:
        // https://github.com/launchbadge/sqlx/blob/5f3245d7f4535a5afaec8de369a931c8293fdb55/tests/postgres/postgres.rs
        let pg_pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await
            .expect("postgres database unavailable");

        pg_pool
    }

    pub async fn create_service(pg_pool: PgPool) -> std::io::Result<()> {
        let static_service_info: &str = Box::leak(service_info().into_boxed_str());

        HttpServer::new(move || {
            App::new()
                .app_data(SafeData::new(pg_pool.clone()))
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
