use crate::{
    schema::*,
    types::{AsyncPool, ProcessError, ProcessResult},
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = nixpkgs_channels_sources)]
pub struct NixpkgsChannelSource {
    pub id: i32,
    pub channel_id: i32,
    pub source_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = nixpkgs_channels_sources)]
pub struct NewNixpkgsChannelSource {
    pub channel_id: i32,
    pub source_id: i32,
}

impl NixpkgsChannelSource {
    pub async fn create_only(
        pool: AsyncPool,
        channel_id: i32,
        source_id: i32,
    ) -> ProcessResult<()> {
        let mut conn = pool.get().await?;
        let new_row = NewNixpkgsChannelSource {
            channel_id,
            source_id,
        };

        diesel::insert_into(nixpkgs_channels_sources::table)
            .values(&new_row)
            .execute(&mut conn)
            .await
            .map(|_| ())
            .map_err(ProcessError::DieselError)
    }
}
