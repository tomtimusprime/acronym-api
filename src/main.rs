use sqlx::MySqlPool;
mod db;
use db::{init_db, all_acronyms};

#[tokio::main]
async fn main() {
    // let DATABASE_URL = "mysql://root:mysqlrootpassword@localhost:3306/acronymsdb";
    // let pool = MySqlPool::connect(&DATABASE_URL).await.expect("Could not connect to the database");
    // let rows = sqlx::query("SELECT * FROM acronymsdb.acronyms WHERE id=1")
    // .fetch_all(&pool) // Pass the pool as a reference
    // .await
    // .expect("Failed to execute query");
    // println!("{:?}", rows);
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
