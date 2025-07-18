use crate::{
    models::{GitHostSource, GitSource, NixpkgsSource, package_version::PackageVersion},
    schema::*,
    types::{AsyncPool, ProcessError, ProcessResult},
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use super::Source;

#[derive(Queryable, Selectable, Associations, Identifiable, Debug, PartialEq)]
#[diesel(belongs_to(PackageVersion))]
#[diesel(belongs_to(NixpkgsSource))]
#[diesel(belongs_to(GitHostSource))]
#[diesel(belongs_to(GitSource))]
#[diesel(table_name = package_versions_sources)]
pub struct PackageVersionSource {
    pub id: i32,
    pub package_version_id: i32,
    pub nixpkgs_source_id: Option<i32>,
    pub git_host_source_id: Option<i32>,
    pub git_source_id: Option<i32>,
}

impl PackageVersionSource {
    pub async fn create(
        pool: AsyncPool,
        package_version: &PackageVersion,
        source: impl Source,
    ) -> ProcessResult<Self> {
        let mut conn = pool.get().await?;
        let mut new_row = NewPackageVersionSource {
            package_version_id: package_version.id,
            ..Default::default()
        };
        source.update_package_version_source_id(&mut new_row);

        diesel::insert_into(package_versions_sources::table)
            .values(&new_row)
            .returning(Self::as_returning())
            .get_result(&mut conn)
            .await
            .map_err(ProcessError::DieselError)
    }
}

#[derive(Insertable, Default)]
#[diesel(table_name = package_versions_sources)]
pub struct NewPackageVersionSource {
    pub package_version_id: i32,
    pub nixpkgs_source_id: Option<i32>,
    pub git_host_source_id: Option<i32>,
    pub git_source_id: Option<i32>,
}
