use crate::{
    models::{GitHostSource, GitSource, NixpkgsSource, package_version::PackageVersion},
    schema::*,
};
use diesel::prelude::*;

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
    pub(crate) fn create(
        conn: &mut PgConnection,
        package_version: &PackageVersion,
        source: &Source,
    ) -> QueryResult<Self> {
        let new_row = NewPackageVersionSource {
            package_version_id: package_version.id,
            ..Default::default()
        }
        .apply_source(source);

        diesel::insert_into(package_versions_sources::table)
            .values(&new_row)
            .returning(Self::as_returning())
            .get_result(conn)
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

impl NewPackageVersionSource {
    pub fn apply_source(mut self, source: &Source) -> Self {
        match source {
            Source::Nixpkgs(nixpkgs) => self.nixpkgs_source_id = Some(nixpkgs.id),
            Source::GitHost(git_host) => self.git_host_source_id = Some(git_host.id),
            Source::Git(git) => self.git_source_id = Some(git.id),
        }

        self
    }
}
