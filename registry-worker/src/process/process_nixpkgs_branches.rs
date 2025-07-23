use futures_util::stream::StreamExt;
use std::sync::Arc;

use octocrab::Octocrab;
use tokio::task::JoinSet;
use tokio_util::either::Either;

use crate::{
    models::{NixpkgsChannel, NixpkgsChannelSource},
    types::{AsyncPool, ProcessError, ProcessResult},
};

pub async fn process_nixpkgs_branches(pool: AsyncPool, oc: Arc<Octocrab>) -> ProcessResult<()> {
    let branches = oc
        .repos("nixos", "nixpkgs")
        .list_branches()
        .per_page(100)
        .send()
        .await
        .map_err(ProcessError::OctocrabError)?;

    let mut set = JoinSet::new();

    branches
        .into_stream(&oc)
        .filter_map(|branch| async {
            branch
                .ok()
                .and_then(|branch| match branch.name.starts_with("nixos-") {
                    true => Some(branch.name),
                    false => None,
                })
        })
        .for_each(|branch| {
            let pool = pool.clone();

            set.spawn(async move {
                let _ = NixpkgsChannel::find_by_name(pool.clone(), &branch)
                    .await
                    .map_or_else(
                        |_| Either::Left(async { NixpkgsChannel::create(pool, &branch).await }),
                        |v| Either::Right(async { Ok(v) }),
                    )
                    .await;
            });

            futures::future::ready(())
        })
        .await;

    set.join_all().await;

    Ok(())
}
