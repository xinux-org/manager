use std::env;

use diesel::prelude::*;
use dotenvy::dotenv;
use process::process_exports;

mod models;
mod process;
mod schema;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut conn = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    let exports = flake_info::process_test(
        "test-single-firefox",
        &flake_info::data::import::Kind::Package,
    )
    .expect("Failed to process");

    process_exports(&mut conn, &exports).await;
}
