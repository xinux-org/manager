use chrono::{DateTime, Days, Utc};
use futures_util::StreamExt;
use itertools::Itertools;
use std::sync::Arc;

use octocrab::Octocrab;
use tokio::task::JoinSet;

use crate::{
    models::NixpkgsChannel,
    types::{AsyncPool, ProcessResult},
};

pub async fn process_nixpkgs_commits(pool: AsyncPool, oc: Arc<Octocrab>) -> ProcessResult<()> {
    let _ = fetch_branch_commits_from_github(pool, oc)
        .await?
        .into_iter()
        .flatten()
        .chunks(8000)
        .into_iter()
        .map(|ch| ch);

    // TODO:
    // 1. create all the commits, ignore existing
    // 1.

    Ok(())
}

async fn fetch_branch_commits_from_github(
    pool: AsyncPool,
    oc: Arc<Octocrab>,
) -> ProcessResult<Vec<Vec<(String, DateTime<Utc>, i32)>>> {
    let mut set = JoinSet::new();

    NixpkgsChannel::get_all(pool.clone())
        .await?
        .into_iter()
        .map(|branch| (branch, oc.clone()))
        .map(|(branch, oc)| async move {
            oc.repos("nixos", "nixpkgs")
                .list_commits()
                .branch(branch.name)
                .per_page(100)
                .since(
                    // TODO:: sinde the latest commit
                    // TODO:: maybe since the
                    // Utc::now()
                    //     .checked_sub_months(Months::new(12 * 3))
                    //     .unwrap_or(Utc::now()),
                    Utc::now()
                        .checked_sub_days(Days::new(1))
                        // .checked_sub_months(Months::new(1))
                        .unwrap_or(Utc::now()),
                )
                .send()
                .await
                .map(|page_commit| (page_commit, branch.id))
        })
        .for_each(|fun| {
            set.spawn(fun);
        });

    let mut set2 = JoinSet::new();

    set.join_all()
        .await
        .into_iter()
        .filter_map(|pages| pages.ok())
        .map(|(pages, branch_id)| (pages, branch_id, oc.clone()))
        .map(|(pages, branch_id, oc)| async move {
            pages
                .into_stream(&oc)
                .filter_map(|result| async { result.ok() })
                .filter_map(|repo_commit| async {
                    repo_commit
                        .commit
                        .author
                        .or(repo_commit.commit.committer)
                        .and_then(|c| c.date)
                        .map(|date| (repo_commit.sha, date, branch_id))
                })
                .collect::<Vec<_>>()
                .await
        })
        .for_each(|fun| {
            set2.spawn(fun);
        });

    Ok(set2.join_all().await)
}
