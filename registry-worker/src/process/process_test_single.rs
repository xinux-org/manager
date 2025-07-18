use tokio_util::either::Either;

use crate::{
    models::{NixpkgsSource, Source},
    process::process_exports,
    types::AsyncPool,
};

pub async fn test_single(pool: AsyncPool) {
    let channel = "test_single";
    let git_ref = "test_single";

    // if !source.is_processed() {
    //     let exports = flake_info::process_test(
    //         "test-single-firefox",
    //         &flake_info::data::import::Kind::Package,
    //     )
    //     .expect("Failed to process");
    //
    //     let result = process_exports(pool.clone(), source.clone(), exports).await;
    //
    //     println!("{:?}", result.into_iter().filter(|v| v.is_err()));
    //
    //     source
    //         .set_processed(pool, true)
    //         .await
    //         .expect("Failed to update processed");
    // }
}
