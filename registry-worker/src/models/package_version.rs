use crate::{
    models::{GitHostSource, GitSource, NixpkgsSource, license::License, package::Package},
    schema::*,
};
use diesel::prelude::*;
use flake_info::data::Derivation;

use super::Source;

#[derive(Queryable, Identifiable, Selectable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(NixpkgsSource))]
#[diesel(belongs_to(GitHostSource))]
#[diesel(belongs_to(GitSource))]
#[diesel(belongs_to(Package))]
#[diesel(belongs_to(License))]
#[diesel(table_name = package_versions)]
pub struct PackageVersion {
    pub id: i32,
    pub nixpkgs_source_id: Option<i32>,
    pub git_host_source_id: Option<i32>,
    pub git_source_id: Option<i32>,
    pub package_id: i32,
    pub license_id: Option<i32>,
    pub available: bool,
    pub broken: bool,
    pub insecure: bool,
    pub changelog: Option<String>,
    pub version: String,
}

#[derive(Insertable, Default)]
#[diesel(table_name = package_versions)]
pub struct NewPackageVersion<'a> {
    pub nixpkgs_source_id: Option<i32>,
    pub git_host_source_id: Option<i32>,
    pub git_source_id: Option<i32>,
    pub package_id: i32,
    pub license_id: Option<i32>,
    pub available: bool,
    pub broken: bool,
    pub insecure: bool,
    pub changelog: Option<&'a str>,
    pub version: &'a str,
}

impl<'a> NewPackageVersion<'a> {
    pub fn apply_source(mut self, source: &Source) -> Self {
        match source {
            Source::Nixpkgs(nixpkgs) => self.nixpkgs_source_id = Some(nixpkgs.id),
            Source::GitHost(git_host) => self.git_host_source_id = Some(git_host.id),
            Source::Git(git) => self.git_source_id = Some(git.id),
        }

        self
    }
}

impl PackageVersion {
    pub fn find_by_package_and_version(
        conn: &mut PgConnection,
        package_id: i32,
        version: &str,
    ) -> QueryResult<Self> {
        use crate::schema::package_versions::dsl;

        dsl::package_versions
            .filter(dsl::package_id.eq(package_id).and(dsl::version.eq(version)))
            .limit(1)
            .select(Self::as_select())
            .first(conn)
    }

    pub fn create_from(
        conn: &mut PgConnection,
        source: &Source,
        package: &Package,
        derivation: &Derivation,
    ) -> QueryResult<Self> {
        if let Derivation::Package {
            package_pversion, ..
        } = derivation
        {
            let new_row = NewPackageVersion {
                package_id: package.id,
                version: package_pversion,
                ..Default::default()
            }
            .apply_source(source);

            diesel::insert_into(package_versions::table)
                .values(&new_row)
                .returning(Self::as_returning())
                .get_result(conn)
        } else {
            unreachable!("package version derivation must be a package!")
        }
    }
}
