use crate::{
    models::{NewPackageVersionSource, Source},
    schema::*,
    types::{AsyncPool, ProcessError, ProcessResult},
};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq, Clone)]
#[diesel(table_name = nixpkgs_sources)]
pub struct NixpkgsSource {
    pub id: i32,
    pub sha: String,
    pub processed: bool,
    pub committed_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub locked_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = nixpkgs_sources)]
pub struct NewNixpkgsSource<'a> {
    pub sha: &'a str,
    pub committed_at: &'a NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = nixpkgs_sources)]
pub struct LockedNixpkgsSource<'a> {
    pub locked_at: Option<&'a NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = nixpkgs_sources)]
pub struct ProcessedNixpkgsSource {
    pub processed: bool,
}

impl NixpkgsSource {
    pub async fn find_by_sha(pool: AsyncPool, sha: &str) -> ProcessResult<Self> {
        let mut conn = pool.get().await?;
        use crate::schema::nixpkgs_sources::dsl;

        dsl::nixpkgs_sources
            .filter(dsl::sha.eq(sha))
            .limit(1)
            .select(Self::as_select())
            .first(&mut conn)
            .await
            .map_err(ProcessError::DieselError)
    }

    pub async fn create(
        pool: AsyncPool,
        sha: &str,
        committed_at: &NaiveDateTime,
    ) -> ProcessResult<Self> {
        let mut conn = pool.get().await?;
        let new_row = NewNixpkgsSource { sha, committed_at };

        diesel::insert_into(nixpkgs_sources::table)
            .values(&new_row)
            .returning(Self::as_returning())
            .get_result(&mut conn)
            .await
            .map_err(ProcessError::DieselError)
    }

    pub async fn update_processed(&self, pool: AsyncPool, processed: bool) -> ProcessResult<()> {
        use crate::schema::nixpkgs_sources::dsl;
        let mut conn = pool.get().await?;

        diesel::update(dsl::nixpkgs_sources)
            .filter(dsl::id.eq(self.id))
            .set(dsl::processed.eq(processed))
            .execute(&mut conn)
            .await
            .map(|_| ())
            .map_err(ProcessError::DieselError)
    }
}

impl Source for NixpkgsSource {
    fn is_processed(&self) -> bool {
        self.processed
    }

    async fn set_processed(&self, pool: AsyncPool, processed: bool) -> ProcessResult<()> {
        self.update_processed(pool, processed).await
    }

    fn update_package_version_source_id(
        &self,
        package_version_source: &mut NewPackageVersionSource,
    ) {
        package_version_source.nixpkgs_source_id = Some(self.id);
    }
}
