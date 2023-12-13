use anyhow::Result;
use serde::{Serialize, Deserialize};
use sqlx::{FromRow, MySqlPool, Row};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Acronym {
    pub id: i32,
    pub acronym: String,
    pub definition: String,
}

pub async fn init_db() -> Result<MySqlPool> {
    let database_url = std::env::var("DATABASE_URL")?;
    let connection_pool = MySqlPool::connect(&database_url).await?;
    //sqlx::migrate!().run(&connection_pool).await?;
    Ok(connection_pool)
}

pub async fn all_acronyms(connection_pool: &MySqlPool) -> Result<Vec<Acronym>> {
    Ok(
        sqlx::query_as::<_, Acronym>("SELECT * FROM acronyms")
        .fetch_all(connection_pool)
        .await?,
    )
}