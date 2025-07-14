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

    test_single(&mut ctx).await;
    // latest_nixpkgs(&mut ctx).await;
}

async fn test_single(ctx: &mut Context) {
    let channel = "test";
    let git_ref = "test";

    let (source, created) =
        NixpkgsSource::find_by_channel_and_ref(&mut ctx.pg_conn, channel, git_ref)
            .map(|source| (source, false))
            .or_else(|_| {
                NixpkgsSource::create(&mut ctx.pg_conn, channel, git_ref)
                    .map(|source| (source, true))
            })
            .map(|(source, created)| (Source::Nixpkgs(source), created))
            .expect("Could not create source");

    if created || !source.is_processed() {
        let exports = flake_info::process_test(
            "test-single-firefox",
            &flake_info::data::import::Kind::Package,
        )
        .expect("Failed to process");

        process_exports(ctx, &source, &exports).await;

        source
            .set_processed(&mut ctx.pg_conn, true)
            .expect("Failed to update processed");
    }
}

async fn latest_nixpkgs(ctx: &mut Context) {
    let nixpkgs_source = flake_info::data::Source::nixpkgs("25.05".to_string())
        .await
        .expect("failed to fetch nixpkgs latest revision");

    let (source, created) = NixpkgsSource::find_by_channel_and_ref(
        &mut ctx.pg_conn,
        &nixpkgs_source.channel,
        &nixpkgs_source.git_ref,
    )
    .map(|source| (source, false))
    .or_else(|_| {
        NixpkgsSource::create(
            &mut ctx.pg_conn,
            &nixpkgs_source.channel,
            &nixpkgs_source.git_ref,
        )
        .map(|source| (source, true))
    })
    .map(|(source, created)| (Source::Nixpkgs(source), created))
    .expect("Could not create source");

    if created || !source.is_processed() {
        let exports = flake_info::process_nixpkgs(
            &flake_info::data::Source::Nixpkgs(nixpkgs_source),
            &flake_info::data::import::Kind::Package,
        )
        .expect("Failed to process");

        process_exports(ctx, &source, &exports).await;

        source
            .set_processed(&mut ctx.pg_conn, true)
            .expect("Failed to update processed");
    }
}
