use futures::future::Either;
use tokio::task::JoinSet;

use crate::{
    libs::super_orm::{CreateAll, FindOrCreate, FindOrCreateAll},
    models::{
        Maintainer, NewPackage, NewPackageVersionPlatform, NewPlatform, PackageVersion,
        PackageVersionMaintainer, PackageVersionSource, Source,
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
                    // package_license,
                    package_pversion,
                    package_platforms,
                    package_maintainers,
                    package_description,
                    mut package_homepage,
                    ..
                } => {
                    println!("Processing {}-{}", &package_pname, &package_pversion);

                    let package = NewPackage::from_values(
                        package_pname,
                        package_description,
                        package_homepage.pop(),
                    )
                    .find_or_create(pool.clone())
                    .await?;

                    let package_version =
                        PackageVersion::find_or_create(pool.clone(), package.id, &package_pversion)
                            .await?;

                    let platforms_vec = package_platforms
                        .into_iter()
                        .map(NewPlatform::from_values)
                        .collect();

                    let platforms =
                        NewPlatform::find_or_create_all(pool.clone(), platforms_vec).await?;

                    let package_version_platforms_vec = platforms
                        .into_iter()
                        .map(|platform| {
                            NewPackageVersionPlatform::from_values(package_version.id, platform.id)
                        })
                        .collect();

                    let _ = NewPackageVersionPlatform::create_all(
                        pool.clone(),
                        &package_version_platforms_vec,
                    )
                    .await?;

                    // let mut set_for_licenses = JoinSet::new();
                    // package_license.into_iter().for_each(|license| {
                    //     let pool = pool.clone();
                    //     println!("Inside the License");
                    //
                    //     set_for_licenses.spawn(async move {
                    //         License::find_by_name(pool.clone(), license.clone())
                    //             .await
                    //             .map_or_else(
                    //                 |_| {
                    //                     Either::Left(async { License::create(pool, license).await })
                    //                 },
                    //                 |v| Either::Right(async { Ok(v) }),
                    //             )
                    //             .await
                    //     });
                    // });

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

                    let maintainers = set_for_maintainers
                        .join_all()
                        .await
                        .into_iter()
                        .filter_map(|r| r.ok())
                        .collect::<Vec<_>>();

                    // let licenses = set_for_licenses
                    //     .join_all()
                    //     .await
                    //     .into_iter()
                    //     .filter_map(|r| r.ok())
                    //     .collect::<Vec<_>>();

                    let source = source.clone();
                    let _ =
                        PackageVersionSource::create(pool.clone(), &package_version, source).await;

                    PackageVersionMaintainer::create_all_only(
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
