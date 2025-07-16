use std::env;

use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use dotenvy::dotenv;

use crate::process::{process_latest_nixpkgs::latest_nixpkgs, process_test_single::test_single};

mod models;
mod process;
mod schema;
mod types;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config =
        AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(&database_url);
    let pool = bb8::Pool::builder()
        .build(config)
        .await
        .unwrap_or_else(|_| panic!("Error connecting to {}", &database_url));

    let _ = tokio::spawn(async move {
        test_single(pool.clone()).await;
        // latest_nixpkgs(pool.clone()).await;
    })
    .await;
}
