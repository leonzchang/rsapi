use sqlx::{postgres::PgRow, PgPool, Row};

// use crate::utils::ensure_affected;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("DB error: `{0}`")]
    Database(#[from] sqlx::Error),

    #[error("reqwest error `{0}`")]
    Reqwest(#[from] reqwest::Error),
}

#[async_trait::async_trait]
pub trait SqlSource {
    async fn update_data(self, person_name: String) -> Result<i32, Error>;

    async fn get_all_data(self) -> Result<Vec<(i32, String)>, Error>;

    async fn post_data(self, person_name: String) -> Result<i32, Error>;
}

#[async_trait::async_trait]
impl SqlSource for &PgPool {
    async fn update_data(self, person_name: String) -> Result<i32, Error> {
        let result = set_data(self, person_name).await?;

        Ok(result)
    }

    async fn get_all_data(self) -> Result<Vec<(i32, String)>, Error> {
        get_all(self).await.map_err(Error::Database)
    }

    async fn post_data(self, person_name: String) -> Result<i32, Error> {
        let result = set_data(self, person_name).await?;

        Ok(result)
    }
}

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

async fn set_data(pg_pool: &PgPool, person_name: String) -> Result<i32, Error> {
    sqlx::query(
        r#"
            INSERT INTO info (person_name) VALUES ($1) RETURNING person_id
        "#,
    )
    .bind(person_name)
    .map(|row: PgRow| Ok(row.get::<i32, _>(0)))
    .fetch_one(pg_pool)
    .await?
    // .and_then(ensure_affected(1))
}

async fn add_data(pg_pool: &PgPool, person_name: String) -> Result<i32, Error> {
    sqlx::query(
        r#"
            INSERT INTO info (person_name) VALUES ($1) RETURNING person_id
        "#,
    )
    .bind(person_name)
    .map(|row: PgRow| Ok(row.get::<i32, _>(0)))
    .fetch_one(pg_pool)
    .await?
    // .and_then(ensure_affected(1))
}
