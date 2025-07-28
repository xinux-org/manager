use diesel_async::async_connection_wrapper::AsyncConnectionWrapper;
use diesel_async::{AsyncConnection, AsyncPgConnection, SimpleAsyncConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use dotenvy::dotenv;
use tokio::sync::OnceCell;

use crate::types::AsyncPool;

#[allow(dead_code)]
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
static INIT: OnceCell<()> = OnceCell::const_new();
const DB_NAME: &str = "registry_testing_db";

#[allow(dead_code)]
fn db_name_to_env_url(db_name: &str) -> String {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found");
    let mut db_url = url::Url::parse(&db_url).expect("Failed to parse DATABASE_URL");
    db_url.set_path(db_name);
    db_url.to_string()
}

async fn diesel_init() {
    // Remove existing first and then create testing database during the connection to the development database
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found");
    let conn = AsyncPgConnection::establish(&db_url)
        .await
        .expect("Failed to create async pg connection");
    let mut conn = AsyncConnectionWrapper::<AsyncPgConnection>::from(conn);

    conn.batch_execute(&format!("DROP DATABASE IF EXISTS {DB_NAME}"))
        .await
        .unwrap_or_else(|err| panic!("Failed to drop database with name {DB_NAME}: {err}"));

    conn.batch_execute(&format!("CREATE DATABASE {DB_NAME}"))
        .await
        .unwrap_or_else(|err| panic!("Failed to create database with name {DB_NAME}: {err}"));

    // Connect to testing database and run all pending migrations
    let db_url = db_name_to_env_url(DB_NAME);
    let conn = AsyncPgConnection::establish(&db_url)
        .await
        .expect("Failed to create async pg connection");
    let mut conn = AsyncConnectionWrapper::<AsyncPgConnection>::from(conn);

    tokio::task::spawn_blocking(move || {
        conn.run_pending_migrations(MIGRATIONS).unwrap();
    })
    .await
    .expect("Failed to run migrations");
}

#[allow(dead_code)]
pub async fn diesel_test_pool() -> AsyncPool {
    dotenv().ok();
    use diesel_async::pooled_connection::AsyncDieselConnectionManager;
    use diesel_async::pooled_connection::bb8::Pool;

    INIT.get_or_init(diesel_init).await;

    let db_url = db_name_to_env_url(DB_NAME);
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(&db_url);
    Pool::builder().max_size(1).build(config).await.unwrap()
}
