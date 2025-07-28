use std::{env, sync::Arc, time::Duration};

use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use dotenvy::dotenv;
use tokio::task::JoinSet;

use crate::{
    logger::SimpleLogger,
    process::{
        process_nixpkgs_branches::process_nixpkgs_branches,
        process_nixpkgs_commits::process_nixpkgs_commits, process_test_single::test_single,
    },
};

mod libs;
mod logger;
mod models;
mod process;
mod schema;
mod types;

static LOGGER: SimpleLogger = SimpleLogger;

#[tokio::main]
async fn main() {
    dotenv().ok();
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(log::LevelFilter::Info))
        .expect("Failed to set up logger");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config =
        AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(&database_url);
    let pool = bb8::Pool::builder()
        .build(config)
        .await
        .unwrap_or_else(|_| panic!("Error connecting to {}", &database_url));

    let oc = octocrab::Octocrab::builder()
        .add_retry_config(octocrab::service::middleware::retry::RetryConfig::Simple(3));

    let oc = if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        oc.personal_token(token).build()
    } else {
        oc.build()
    };

    let oc = Arc::new(oc.expect("Failed to build octocrab"));

    let mut set = JoinSet::new();

    {
        let pool = pool.clone();
        set.spawn(async move {
            log::info!("test-single: processing");

            match test_single(pool.clone()).await {
                Ok(_) => {
                    log::info!("test-single: processed successfully");
                }
                Err(err) => {
                    log::error!("test-single: {:?}", err);
                }
            };
        });
    }

    {
        let pool = pool.clone();
        let oc = oc.clone();
        set.spawn(async move {
            loop {
                log::info!("nixpkgs-branches: updating");

                match process_nixpkgs_branches(pool.clone(), oc.clone()).await {
                    Ok(_) => {
                        log::info!("nixpkgs-branches: all branches updated");
                    }
                    Err(err) => {
                        log::error!("nixpkgs-branches: {:?}", err);
                    }
                };

                // once a day
                tokio::time::sleep(Duration::from_secs(60 * 60 * 24)).await;
            }
        });
    }

    {
        let pool = pool.clone();
        let oc = oc.clone();
        set.spawn(async move {
            loop {
                log::info!("nixpkgs-commits: updating");
                match process_nixpkgs_commits(pool.clone(), oc.clone()).await {
                    Ok(_) => {
                        log::info!("nixpkgs-commits: all branch commits updated");
                    }
                    Err(err) => {
                        log::error!("nixpkgs-commits: {:?}", err);
                    }
                };

                // once in an hour
                tokio::time::sleep(Duration::from_secs(60 * 60)).await;
            }
        });
    }

    set.join_all().await;
}
