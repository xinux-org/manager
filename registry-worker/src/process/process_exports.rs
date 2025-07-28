use tokio::task::JoinSet;

use crate::{
    libs::super_orm::{CreateAll, FindOrCreate, FindOrCreateAll}, models::{
        NewLicense, NewMaintainer, NewPackage, NewPackageVersionMaintainer, NewPackageVersionPlatform, NewPlatform, PackageVersion, PackageVersionSource, Source
    },
    types::{AsyncPool, ProcessError, ProcessResult}
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

                    let maintainers_vec = package_maintainers
                        .into_iter()
                        .map(|maintainer| 
                            NewMaintainer::from_values(
                                maintainer.name,
                                maintainer.github,
                                maintainer.email
                            ))
                        .collect();

                    let maintainers =
                        NewMaintainer::find_or_create_all(pool.clone(), maintainers_vec).await?;

                    let package_version_maintainers_vec = maintainers
                        .into_iter()
                        .map(|maintainer| {
                            NewPackageVersionMaintainer::from_values(package_version.id, maintainer.id)
                        })
                        .collect();

                    let licenses_vec = package_license
                        .into_iter()
                       .map(|license| 
                            NewLicense::from_values(
                                license.fullName,
                                license.url
                            ))
                        .collect();
                    
                    let _ =
                        NewLicense::find_or_create_all(pool.clone(), licenses_vec).await?;


                    let _ = NewPackageVersionPlatform::create_all(
                        pool.clone(),
                        &package_version_platforms_vec,
                    )
                    .await?;

                    let _ = NewPackageVersionMaintainer::create_all(
                        pool.clone(),
                        &package_version_maintainers_vec,
                    )
                    .await?;

                    let source = source.clone();
                    let _ =
                        PackageVersionSource::create(pool.clone(), &package_version, source).await;

                    Ok(())
                }
                _ => Err(ProcessError::NotImplemented),
            }
        });
    });

    set.join_all().await
}
