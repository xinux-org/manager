use diesel::PgConnection;
use moka::sync::Cache;

use crate::{
    context::Context,
    models::{Platform, create_platform, find_platform_by_name},
};

pub fn process_platforms(
    conn: &mut PgConnection,
    cache: &mut Cache<String, Platform>,
    platforms: &[String],
) -> Vec<Platform> {
    platforms
        .iter()
        .filter_map(|name| {
            cache.get(name).or_else(|| {
                find_platform_by_name(conn, name)
                    .or_else(|| create_platform(conn, name))
                    .and_then(|platform| {
                        cache.insert(name.clone(), platform);
                        cache.get(name)
                    })
            })
        })
        .collect()
}

pub async fn process_exports(ctx: &mut Context, exports: &Vec<flake_info::data::Export>) {
    for export in exports {
        match &export.item {
            flake_info::data::Derivation::Package {
                package_platforms, ..
            } => {
                let p = process_platforms(
                    &mut ctx.pg_conn,
                    &mut ctx.platforms_cache,
                    package_platforms,
                );
                println!("{:?}", package_platforms);
                println!("{:?}", p);
            }
            _ => todo!(),
        };

        println!("{:?}", export);
    }
}
