use crate::{
    schema::*,
    types::{AsyncPool, ProcessError, ProcessResult},
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq, Clone)]
#[diesel(table_name = nixpkgs_sources)]
pub struct NixpkgsSource {
    pub id: i32,
    pub channel: String,
    pub git_ref: String,
    pub processed: bool,
}

#[derive(Insertable)]
#[diesel(table_name = nixpkgs_sources)]
pub struct NewNixpkgSource<'a> {
    pub channel: &'a str,
    pub git_ref: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = nixpkgs_sources)]
pub struct ProcessedNixpkgSource {
    pub processed: bool,
}

impl NixpkgsSource {
    pub async fn find_by_channel(pool: AsyncPool, channel: &str) -> ProcessResult<Self> {
        use crate::schema::nixpkgs_sources::dsl;

        let mut conn = pool.get().await?;

        dsl::nixpkgs_sources
            .filter(dsl::channel.eq(channel))
            .limit(1)
            .select(Self::as_select())
            .first(&mut conn)
            .await
            .map_err(ProcessError::DieselError)
    }

    pub async fn find_by_channel_and_ref(
        pool: AsyncPool,
        channel: &str,
        git_ref: &str,
    ) -> ProcessResult<Self> {
        let mut conn = pool.get().await?;
        use crate::schema::nixpkgs_sources::dsl;

        dsl::nixpkgs_sources
            .filter(dsl::channel.eq(channel).and(dsl::git_ref.eq(git_ref)))
            .limit(1)
            .select(Self::as_select())
            .first(&mut conn)
            .await
            .map_err(ProcessError::DieselError)
    }

    pub async fn create(pool: AsyncPool, channel: &str, git_ref: &str) -> ProcessResult<Self> {
        let mut conn = pool.get().await?;
        let new_row = NewNixpkgSource { channel, git_ref };

        diesel::insert_into(nixpkgs_sources::table)
            .values(&new_row)
            .returning(Self::as_returning())
            .get_result(&mut conn)
            .await
            .map_err(ProcessError::DieselError)
    }

    pub async fn update_processed(self, pool: AsyncPool, processed: bool) -> ProcessResult<()> {
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
