use tokio_util::either::Either;

use crate::{
    models::{NixpkgsSource, Source},
    process::process_exports,
    types::AsyncPool,
};

pub async fn latest_nixpkgs(pool: AsyncPool) {
    let nixpkgs_source = flake_info::data::Source::nixpkgs("25.05".to_string())
        .await
        .expect("failed to fetch nixpkgs latest revision");

    let source = NixpkgsSource::find_by_channel_and_ref(
        pool.clone(),
        &nixpkgs_source.channel,
        &nixpkgs_source.git_ref,
    )
    .await
    .map_or_else(
        |_| {
            Either::Left(async {
                NixpkgsSource::create(
                    pool.clone(),
                    &nixpkgs_source.channel,
                    &nixpkgs_source.git_ref,
                )
                .await
            })
        },
        |v| Either::Right(async { Ok(v) }),
    )
    .await
    .map(Source::Nixpkgs)
    .expect("Could not create source");

    println!("{:?}", source);

    if !source.is_processed() {
        let exports = flake_info::process_nixpkgs(
            &flake_info::data::Source::Nixpkgs(nixpkgs_source),
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
}
