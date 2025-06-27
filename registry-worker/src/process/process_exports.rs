use crate::{
    context::Context,
    models::{Package, PackageVersion, PackageVersionPlatform, Platform, Source},
};

pub async fn process_exports(
    ctx: &mut Context,
    source: &Source,
    exports: &Vec<flake_info::data::Export>,
) {
    for export in exports {
        match &export.item {
            flake_info::data::Derivation::Package {
                package_pname,
                package_pversion,
                package_platforms,
                package_description,
                package_homepage,
                ..
            } => {
                let _ = Package::find_by_name(&mut ctx.pg_conn, package_pname)
                    .or_else(|_| {
                        Package::create(
                            &mut ctx.pg_conn,
                            package_pname,
                            package_description.clone(),
                            package_homepage.clone().pop(),
                        )
                    })
                    .inspect(|package| {
                        let _ = PackageVersion::find_by_package_and_version(
                            &mut ctx.pg_conn,
                            package.id,
                            package_pversion,
                        )
                        .inspect_err(|_| {
                            let platforms = package_platforms
                                .iter()
                                .filter_map(|name| {
                                    Platform::cached_get_or_create(
                                        &mut ctx.pg_conn,
                                        &mut ctx.platforms_cache,
                                        name,
                                    )
                                    .ok()
                                })
                                .collect::<Vec<Platform>>();

                            let _ = PackageVersion::create_from(
                                &mut ctx.pg_conn,
                                source,
                                package,
                                &export.item,
                            )
                            .inspect(|package_version| {
                                let _ = PackageVersionPlatform::create_all_only(
                                    &mut ctx.pg_conn,
                                    package_version,
                                    &platforms,
                                );
                            })
                            .expect("why not to create package version platforms?");
                        });
                    })
                    .expect("why not to create package versions?");
            }
            _ => todo!(),
        };

        println!("{:?}", export);
    }
}
