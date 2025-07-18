use std::{env, sync::Arc, time::Duration};

use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use dotenvy::dotenv;

use crate::{logger::SimpleLogger, process::process_nixpkgs_commits::process_nixpkgs_commits};

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

    let oc = octocrab::Octocrab::builder();

    let oc = if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        oc.personal_token(token).build()
    } else {
        oc.build()
    };

    let oc = Arc::new(oc.expect("Failed to build octocrab"));

    tokio::join!(
        async move {
            loop {
                log::info!(target: "nixos-branches", "Updating all nixos branches");

                // match process_nixpkgs_branches(pool.clone(), oc.clone()).await {
                //     Ok(_) => {
                //         println!("N")
                //     }
                //     Err(err) => {
                //         println!("Error occurred while updating all nixos branches {:?}", err)
                //     }
                // };

                // once a day
                tokio::time::sleep(Duration::from_secs(60 * 60 * 24)).await;
            }
            // test_single(pool.clone()).await;
            // latest_nixpkgs(pool.clone()).await;
        },
        async move {
            loop {
                log::info!(target: "nixos-commits", "Updating all nixos commits");
                println!("Updating nixos stable commits");
                process_nixpkgs_commits().await;

                // once in an hour
                tokio::time::sleep(Duration::from_secs(60 * 60)).await;
            }
        },
    );
}
