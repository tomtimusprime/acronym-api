use anyhow::Result;
use serde::{Serialize, Deserialize};
use sqlx::{FromRow, MySqlPool, Row, Error};
use utoipa::ToSchema;

#[derive(Debug, Default, Serialize, Deserialize, FromRow, Clone, ToSchema)]
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

pub async fn acronym_by_id(connection_pool: &MySqlPool, id:i32) -> Result<Acronym> {
    Ok(
        sqlx::query_as::<_, Acronym>("SELECT * FROM acronyms WHERE id=?")
        .bind(id)
        .fetch_one(connection_pool)
        .await?,
    )
}

pub async fn add_acronym<S: ToString>(connection_pool: &MySqlPool, acronym: S, definition: S) -> Result<i32> {
    let acronym = acronym.to_string();
    let definition = definition.to_string();
    Ok(
        sqlx::query("INSERT INTO acronyms (acronym, definition) VALUES (?, ?)")
        .bind(acronym)
        .bind(definition)
        .fetch_one(connection_pool)
        .await?
        .get(0)
    )
}

pub async fn update_acronym(connection_pool: &MySqlPool, acronym: &Acronym) -> Result<()> {
    sqlx::query("UPDATE acronyms SET acronym=?, definition=? WHERE id=?")
    .bind(&acronym.acronym)
    .bind(&acronym.definition)
    .bind(&acronym.id)
    .execute(connection_pool)
    .await?;
    Ok(())
}

pub async fn delete_acronym(connection_pool: &MySqlPool, id:i32) -> Result<()> {
    sqlx::query("DELETE FROM acronyms WHERE id=?")
    .bind(id)
    .execute(connection_pool)
    .await?;
    Ok(())
}

pub async fn search_acronyms(connection_pool: &MySqlPool, search_term: &str) -> Result<Vec<Acronym>, sqlx::Error> {
    sqlx::query_as::<_, Acronym>("SELECT * FROM acronyms WHERE acronym LIKE ? OR definition LIKE ?")
    .bind(format!("%{}%", search_term))
    .bind(format!("%{}%", search_term))
    .fetch_all(connection_pool)
    .await
}