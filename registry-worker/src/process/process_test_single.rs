use chrono::Utc;
use tokio_util::either::Either;

use crate::{
    models::{NixpkgsSource, Source},
    process::process_exports,
    types::{AsyncPool, ProcessResult},
};

pub async fn test_single(pool: AsyncPool) -> ProcessResult<()> {
    // let channel = "test_single";
    let git_ref = "test_single";

    let exports = flake_info::process_test(
        "test-single-firefox",
        &flake_info::data::import::Kind::Package,
    )
    .expect("Failed to process");

    let source = NixpkgsSource::find_by_sha(pool.clone(), git_ref)
        .await
        .map_or_else(
            |_| {
                Either::Left(async {
                    NixpkgsSource::create(pool.clone(), git_ref, &Utc::now().naive_utc()).await
                })
            },
            |v| Either::Right(async { Ok(v) }),
        )
        .await?;

    let result = process_exports(pool.clone(), source.clone(), exports).await;

    println!("{:?}", result.into_iter().filter(|v| v.is_err()));

    source
        .set_processed(pool, true)
        .await
        .expect("Failed to update processed");

    Ok(())
}
