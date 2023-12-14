use sqlx::MySqlPool;
mod db;
use db::{init_db, all_acronyms};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let pool = match init_db().await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to initialize the database: {}", e);
            return;
        }
    };

    match all_acronyms(&pool).await {
        Ok(acronyms) => {
            for acronym in acronyms {
                println!("{:?}", acronym);
            }
        }
        Err(e) => eprintln!("Failed to fetch acronyms: {}", e),
    }

}