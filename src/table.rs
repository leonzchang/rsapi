use sqlx::{postgres::PgRow, PgPool, Row};

use crate::utils::ensure_affected;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("DB error: `{0}`")]
    Database(#[from] sqlx::Error),

    #[error("reqwest error `{0}`")]
    Reqwest(#[from] reqwest::Error),
}

#[async_trait::async_trait]
pub trait SqlSource {
    async fn update_data(self, person_id: i32, person_name: String) -> Result<(), Error>;

    async fn get_all_data(self) -> Result<Vec<(i32, String)>, Error>;
}

#[async_trait::async_trait]
impl SqlSource for &PgPool {
    async fn update_data(self, person_id: i32, person_name: String) -> Result<(), Error> {
        set_data(self, person_id, person_name).await?;

        Ok(())
    }

    async fn get_all_data(self) -> Result<Vec<(i32, String)>, Error> {
        get_all(self).await.map_err(Error::Database)
    }
}

//get_all 去 query database 拿到 rng_data table 中 rng, sig, chain_id 三個欄位的資料
async fn get_all(pg_pool: &PgPool) -> sqlx::Result<Vec<(i32, String)>> {
    sqlx::query(
        r#"
        SELECT person_id, person_name FROM info ORDER BY person_id ASC
    "#,
    )
    .try_map(|row: PgRow| Ok((row.try_get::<i32, _>(0)?, row.try_get::<String, _>(1)?)))
    .fetch_all(pg_pool)
    .await
}

async fn set_data(pg_pool: &PgPool, person_id: i32, person_name: String) -> sqlx::Result<()> {
    sqlx::query(
        r#"
        INSERT INTO info (person_id, person_name) VALUES ($1, $2)
    "#,
    )
    .bind(person_id)
    .bind(person_name)
    .execute(pg_pool)
    .await
    .and_then(ensure_affected(1))
}
