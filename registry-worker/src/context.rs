use diesel::PgConnection;
use moka::sync::Cache;

use crate::models::Platform;

pub struct Context {
    pub pg_conn: PgConnection,
    pub platforms_cache: Cache<String, Platform>,
}

impl Context {
    pub fn new(pg_conn: PgConnection) -> Self {
        Self {
            pg_conn,
            platforms_cache: Cache::new(100),
        }
    }
}
