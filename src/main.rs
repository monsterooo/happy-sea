use sea_orm::{ConnectionTrait, Database, DbBackend, DbErr, Statement};
use dotenv::dotenv;

async fn run() -> Result<(), DbErr> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let db_name = std::env::var("DB_NAME").expect("DB_NAME must be set.");
    let db = Database::connect(&database_url).await?;

    let db = &match db.get_database_backend() {
        DbBackend::Postgres => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE \"{}\";", db_name),
            )).await?;
            let url = format!("{}/{}", database_url, db_name);
            Database::connect(&url).await?
        },
        _ => panic!("Dont's impl"),
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        panic!("{}", err);
    }
}
