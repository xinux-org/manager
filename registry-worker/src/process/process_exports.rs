use tokio::task::JoinSet;
use tokio_util::either::Either;

use crate::{
    models::{
        License, Maintainer, Package, PackageVersion, PackageVersionMaintainer,
        PackageVersionPlatform, PackageVersionSource, Platform, Source,
    }, types::{AsyncPool, ProcessError, ProcessResult}
};

pub async fn process_exports<S>(
    pool: AsyncPool,
    source: S,
    exports: Vec<flake_info::data::Export>,
) -> Vec<ProcessResult<()>>
where
    S: Source + Clone + std::marker::Send + 'static,
{
    // let platforms_cache: Cache<String, Platform> = Cache::new(100);
    let mut set = JoinSet::new();

    exports.into_iter().for_each(|export| {
        let pool = pool.clone();
        let source = source.clone();
        set.spawn(async {
            match export.item {
                flake_info::data::Derivation::Package {
                    package_pname,
                    package_license,
                    package_pversion,
                    package_platforms,
                    package_maintainers,
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

                                                let mut set_for_licenses = JoinSet::new();
                                                package_license.into_iter().for_each(|license| {
                                                    let pool = pool.clone();
                                                    println!("Inside the License");

                                                    set_for_licenses.spawn(async move {
                                                        License::find_by_name(
                                                        pool.clone(),
                                                        license.clone()

                                                    )
                                                            .await
                                                            .map_or_else(
                                                                |_| Either::Left(async { License::create(pool, license).await }),
                                                                |v| Either::Right(async { Ok(v) }),
                                                            )
                                                            .await
                                                    });
                                                });

                                                let mut set_for_maintainers = JoinSet::new();
                                                package_maintainers.into_iter().for_each(|maintainer| {
                                                    let pool = pool.clone();
                                                    println!("Insite the Maintainer");
                                                    set_for_maintainers.spawn(async move {
                                                        Maintainer::find_by_maintainer(
                                                            pool.clone(),
                                                            maintainer.clone()
                                                            // added clone
                                                        )
                                                            .await
                                                            .map_or_else(
                                                                |_|Either::Left(async {
                                                                    Maintainer::create(
                                                                        pool.clone(),
                                                                        maintainer
                                                                    ).await
                                                                }),
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

                                                let maintainers = set_for_maintainers
                                                    .join_all()
                                                    .await
                                                    .into_iter()
                                                    .filter_map(|r| r.ok())
                                                    .collect::<Vec<_>>();

                                                let licenses = set_for_licenses
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
                                                            let source = source.clone();
                                                            let _ = PackageVersionSource::create(
                                                                pool.clone(),
                                                                &package_version,
                                                                source,
                                                            )
                                                            .await;

                                                            let _ = PackageVersionPlatform::create_all_only(
                                                                pool.clone(),
                                                                &package_version,
                                                                &platforms,
                                                            ).await;

                                                            let _ = PackageVersionMaintainer::create_all_only(
                                                                pool.clone(),
                                                                &package_version,
                                                                &maintainers,
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
