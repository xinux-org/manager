use crate::schema::{package_versions_maintainers::maintainer_id, *};
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = revisions)]
pub struct Source {
    pub id: i32,
}

#[derive(Queryable, Selectable, Associations, Identifiable, Debug, PartialEq)]
#[diesel(belongs_to(Source))]
#[diesel(table_name = revisions)]
pub struct Revision {
    pub id: i32,
    pub source_id: i32,
}

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = packages)]
pub struct Package {
    pub id: i32,
}

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = licenses)]
pub struct License {
    pub id: i32,
    pub name: String,
    pub license: Option<String>,
    pub fullname: Option<String>,
    pub shortname: Option<String>,
    pub url: Option<String>,
}

#[derive(Queryable, Identifiable, Selectable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Revision))]
#[diesel(belongs_to(Package))]
#[diesel(belongs_to(License))]
#[diesel(table_name = package_versions)]
pub struct PackageVersion {
    pub id: i32,
    pub revision_id: i32,
    pub package_id: i32,
    pub license_id: i32,
    pub available: bool,
    pub broken: bool,
    pub insecure: bool,
    pub changelog: String,
    pub version: String,
}

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = platforms)]
pub struct Platform {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = platforms)]
pub struct NewPlatform<'a> {
    pub id: i32,
    pub name: &'a str,
}

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = maintainers)]
pub struct Maintainer {
    pub id: i32,
    pub name: String,
    pub github: Option<String>,
    pub email: Option<String>,
}

#[derive(Queryable, Selectable, Associations, Identifiable, Debug, PartialEq)]
#[diesel(belongs_to(PackageVersion))]
#[diesel(belongs_to(Platform))]
#[diesel(table_name = package_versions_platforms)]
pub struct PackageVersionPlatform {
    pub id: i32,
    pub package_version_id: i32,
    pub platform_id: i32,
}
#[derive(Queryable, Selectable, Associations, Identifiable, Debug, PartialEq)]
#[diesel(belongs_to(PackageVersion))]
#[diesel(belongs_to(Maintainer))]
#[diesel(table_name = package_versions_maintainers)]
pub struct PackageVersionMaintainer {
    pub id: i32,
    pub package_version_id: i32,
    pub maintainer_id: i32,
}
