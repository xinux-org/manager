use crate::{
    schema::*,
    types::{AsyncPool, ProcessError, ProcessResult},
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use futures::future::Either;
use moka::sync::Cache;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq, Clone)]
#[diesel(table_name = platforms)]
pub struct Platform {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = platforms)]
pub struct NewPlatform<'a> {
    pub name: &'a str,
}

impl Platform {
    pub async fn find_by_name(pool: AsyncPool, name: &str) -> ProcessResult<Self> {
        use crate::schema::platforms::dsl;
        let mut conn = pool.get().await?;

        dsl::platforms
            .filter(dsl::name.eq(name))
            .limit(1)
            .select(Self::as_select())
            .first(&mut conn)
            .await
            .map_err(ProcessError::DieselError)
    }

    pub async fn create(pool: AsyncPool, name: &str) -> ProcessResult<Self> {
        let mut conn = pool.get().await?;
        let new_row = NewPlatform { name };

        diesel::insert_into(platforms::table)
            .values(&new_row)
            .returning(Self::as_returning())
            .get_result(&mut conn)
            .await
            .map_err(ProcessError::DieselError)
    }

    pub async fn cached_get_or_create(
        pool: AsyncPool,
        cache: &mut Cache<String, Self>,
        name: &str,
    ) -> ProcessResult<Self> {
        cache
            .get(name)
            .map_or_else(
                || {
                    Either::Left(async {
                        Self::find_by_name(pool.clone(), name)
                            .await
                            .map_or_else(
                                |_| Either::Left(async { Self::create(pool, name).await }),
                                |v| Either::Right(async { Ok(v) }),
                            )
                            .await
                            .and_then(|platform| {
                                cache.insert(name.to_string(), platform);
                                cache.get(name).ok_or(ProcessError::DieselError(
                                    diesel::result::Error::NotFound,
                                ))
                            })
                    })
                },
                |v| Either::Right(async { Ok(v) }),
            )
            .await
    }
}
