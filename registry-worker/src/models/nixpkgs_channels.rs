use crate::{
    models::{NewPackageVersionSource, Source},
    schema::*,
    types::{AsyncPool, ProcessError, ProcessResult},
};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq, Clone)]
#[diesel(table_name = nixpkgs_channels)]
pub struct NixpkgsChannel {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = nixpkgs_channels)]
pub struct NewNixpkgsChannel<'a> {
    pub name: &'a str,
}

impl NixpkgsChannel {
    pub async fn find_by_name(pool: AsyncPool, name: &str) -> ProcessResult<Self> {
        let mut conn = pool.get().await?;
        use crate::schema::nixpkgs_channels::dsl;

        dsl::nixpkgs_channels
            .filter(dsl::name.eq(name))
            .limit(1)
            .select(Self::as_select())
            .first(&mut conn)
            .await
            .map_err(ProcessError::DieselError)
    }

    pub async fn create(pool: AsyncPool, name: &str) -> ProcessResult<Self> {
        let mut conn = pool.get().await?;
        let new_row = NewNixpkgsChannel { name };

        diesel::insert_into(nixpkgs_channels::table)
            .values(&new_row)
            .returning(Self::as_returning())
            .get_result(&mut conn)
            .await
            .map_err(ProcessError::DieselError)
    }
}
