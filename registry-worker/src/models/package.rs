use crate::{
    schema::*,
    types::{AsyncPool, ProcessError, ProcessResult},
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use futures::future::Either;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq, Clone)]
#[diesel(table_name = packages)]
pub struct Package {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub homepage: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = packages)]
pub struct NewPackage<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub homepage: Option<&'a str>,
}

impl Package {
    pub async fn find_or_create(
        pool: AsyncPool,
        name: &str,
        description: Option<&str>,
        homepage: Option<&str>,
    ) -> ProcessResult<Self> {
        Self::find_by_name(pool.clone(), name)
            .await
            .map_or_else(
                |_| Either::Left(async { Self::create(pool, name, description, homepage).await }),
                |v| Either::Right(async { Ok(v) }),
            )
            .await
    }

    pub async fn find_by_name(pool: AsyncPool, name: &str) -> ProcessResult<Self> {
        use crate::schema::packages::dsl;
        let mut conn = pool.get().await?;

        dsl::packages
            .filter(dsl::name.eq(name))
            .limit(1)
            .select(Self::as_select())
            .first(&mut conn)
            .await
            .map_err(ProcessError::DieselError)
    }

    pub async fn create(
        pool: AsyncPool,
        name: &str,
        description: Option<&str>,
        homepage: Option<&str>,
    ) -> ProcessResult<Self> {
        let mut conn = pool.get().await?;
        let new_row = NewPackage {
            name,
            description,
            homepage,
        };

        diesel::insert_into(packages::table)
            .values(&new_row)
            .returning(Self::as_returning())
            .get_result(&mut conn)
            .await
            .map_err(ProcessError::DieselError)
    }
}
