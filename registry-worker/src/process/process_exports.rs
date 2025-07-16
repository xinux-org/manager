use tokio::task::JoinSet;
use tokio_util::either::Either;

use crate::{
    models::{
        Package, PackageVersion, PackageVersionPlatform, PackageVersionSource, Platform, Source,
    },
    types::{AsyncPool, ProcessError, ProcessResult},
};

pub async fn process_exports(
    pool: AsyncPool,
    source: Source,
    exports: Vec<flake_info::data::Export>,
) -> Vec<ProcessResult<()>> {
    // let platforms_cache: Cache<String, Platform> = Cache::new(100);
    let mut set = JoinSet::new();

    exports.into_iter().for_each(|export| {
        let pool = pool.clone();
        let source = source.clone();
        set.spawn(async {
            match export.item {
                flake_info::data::Derivation::Package {
                    package_pname,
                    package_pversion,
                    package_platforms,
                    ref package_description,
                    ref package_homepage,
                    ..
                } => {
        println!("Processing {}-{}", &package_pname, &package_pversion);
                    Package::find_by_name(pool.clone(), &package_pname)
                        .await
                        .map_or_else(
                            |_| {
                                Either::Left(async {
                                    Package::create(
                                        pool.clone(),
                                        &package_pname,
                                        package_description.clone(),
                                        package_homepage.clone().pop(),
                                    )
                                    .await
                                })
                            },
                            |v| Either::Right(async { Ok(v) }),
                        )
                        .await
                        .map_or_else(
                            |e| Either::Left(async {Err(e)}),
                            |package| {
                                Either::Right(async move {
                                    PackageVersion::find_by_package_and_version(
                                        pool.clone(),
                                        package.id,
                                        &package_pversion,
                                    )
                                    .await
                                    .map_or_else(
                                        |e| {
                                            Either::Left(async move {
                                                let mut set = JoinSet::new();
                                                package_platforms.into_iter().for_each(|name| {
                                                    let pool = pool.clone();
                                                    set.spawn(async move {
                                                        Platform::find_by_name(pool.clone(), &name)
                                                            .await
                                                            .map_or_else(
                                                                |_| Either::Left(async { Platform::create(pool, &name).await }),
                                                                |v| Either::Right(async { Ok(v) }),
                                                            )
                                                            .await
                                                    });
                                                });

                                                let platforms = set
                                                    .join_all()
                                                    .await
                                                    .into_iter()
                                                    .filter_map(|r| r.ok())
                                                    .collect::<Vec<_>>();

                                                let _ = PackageVersion::create_from(
                                                    pool.clone(),
                                                    package.id,
                                                    &package_pversion,
                                                )
                                                .await
                                                .map_or_else(
                                                    |_| Either::Left(async {}),
                                                    |package_version| {
                                                        Either::Right(async move {
                                                            let _ = PackageVersionSource::create(
                                                                pool.clone(),
                                                                &package_version,
                                                                &source,
                                                            )
                                                            .await;

                                                            let _ = PackageVersionPlatform::create_all_only(
                                                                pool.clone(),
                                                                &package_version,
                                                                &platforms,
                                                            ).await;
                                                        })
                                                    },
                                                )
                                                .await;

                                                Err(e)
                                            })
                                        },
                                        |_| Either::Right(async { Ok(()) }),
                                    )
                                    .await
                                })
                            },
                        ).await
                }
                _ => Err(ProcessError::NotImplemented),
            }
        });
    });

    set.join_all().await
}
