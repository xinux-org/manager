use diesel::PgConnection;

use crate::models::{Platform, create_platform, find_platform_by_name};

pub async fn process_platforms(conn: &mut PgConnection, platforms: &[String]) -> Vec<Platform> {
    platforms
        .iter()
        .filter_map(|name| find_platform_by_name(conn, name).or(create_platform(conn, name)))
        .collect::<Vec<_>>()
}

pub async fn process_exports(conn: &mut PgConnection, exports: &Vec<flake_info::data::Export>) {
    for export in exports {
        match &export.item {
            flake_info::data::Derivation::Package {
                package_platforms, ..
            } => {
                let p = process_platforms(conn, package_platforms).await;
                println!("{:?}", package_platforms);
                println!("{:?}", p);
            }
            _ => todo!(),
        };

        println!("{:?}", export);
    }
}
