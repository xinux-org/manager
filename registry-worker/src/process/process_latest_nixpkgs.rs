use flake_info::data::Nixpkgs;

use crate::{
    models::{NixpkgsSource, Source},
    process::process_exports,
    types::AsyncPool,
};

pub async fn nixpkgs_source(pool: AsyncPool, channel: (), source: NixpkgsSource) {
    if source.is_processed() {
        return;
    }

    let exports = flake_info::process_nixpkgs(
        &flake_info::data::Source::Nixpkgs(Nixpkgs {
            // channel: channel.name,
            channel: "".to_string(),
            git_ref: source.sha.clone(),
        }),
        &flake_info::data::import::Kind::Package,
    )
    .expect("Failed to process");

    let result = process_exports(pool.clone(), source.clone(), exports).await;
    println!("{:?}", result.into_iter().filter(|v| v.is_err()));

    source
        .set_processed(pool, true)
        .await
        .expect("Failed to update processed");
}
