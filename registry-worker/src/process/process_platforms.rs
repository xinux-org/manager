use diesel::PgConnection;
use moka::sync::Cache;

use crate::models::{Platform, create_platform, find_platform_by_name};

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
