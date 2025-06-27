use std::env;

use context::Context;
use diesel::prelude::*;
use dotenvy::dotenv;
use models::{NixpkgsSource, Source};
use process::process_exports;

mod context;
mod models;
mod process;
mod schema;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    let mut ctx = Context::new(conn);

    let channel = "test";
    let git_ref = "test";

    let source = NixpkgsSource::find_by_channel_and_ref(&mut ctx.pg_conn, channel, git_ref)
        .or_else(|_| NixpkgsSource::create(&mut ctx.pg_conn, channel, git_ref))
        .map(Source::Nixpkgs)
        .expect("Could not create source");

    let exports = flake_info::process_test(
        "test-single-firefox",
        &flake_info::data::import::Kind::Package,
    )
    .expect("Failed to process");

    process_exports(&mut ctx, &source, &exports).await;
}
