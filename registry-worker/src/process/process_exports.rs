use futures::future::Either;
use tokio::task::JoinSet;

use crate::{
    models::{
        License, Maintainer, Package, PackageVersion, PackageVersionMaintainer,
        PackageVersionPlatform, PackageVersionSource, Platform, Source, package,
    },
    types::{AsyncPool, ProcessError, ProcessResult},
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
        set.spawn(async move {
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

                    let package = Package::find_or_create(
                        pool.clone(),
                        &package_pname,
                        package_description.as_deref(),
                        package_homepage.get(0).map(|s| s.as_ref()),
                    )
                    .await?;

                    let package_version =
                        PackageVersion::find_or_create(pool.clone(), package.id, &package_pversion)
                            .await?;

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
                            License::find_by_name(pool.clone(), license.clone())
                                .await
                                .map_or_else(
                                    |_| {
                                        Either::Left(async { License::create(pool, license).await })
                                    },
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
                                maintainer.clone(), // added clone
                            )
                            .await
                            .map_or_else(
                                |_| {
                                    Either::Left(async {
                                        Maintainer::create(pool.clone(), maintainer).await
                                    })
                                },
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

                    let source = source.clone();
                    let _ =
                        PackageVersionSource::create(pool.clone(), &package_version, source).await;

                    let _ = PackageVersionPlatform::create_all_only(
                        pool.clone(),
                        &package_version,
                        &platforms,
                    )
                    .await;

                    let _ = PackageVersionMaintainer::create_all_only(
                        pool.clone(),
                        &package_version,
                        &maintainers,
                    )
                    .await?;

                    Ok(())
                }
                _ => Err(ProcessError::NotImplemented),
            }
        });
    });

    set.join_all().await
}
