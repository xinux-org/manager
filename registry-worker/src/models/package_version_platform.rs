use crate::{
    models::{package_version::PackageVersion, platform::Platform},
    schema::*,
};
use diesel::prelude::*;

#[derive(Queryable, Selectable, Associations, Identifiable, Debug, PartialEq)]
#[diesel(belongs_to(PackageVersion))]
#[diesel(belongs_to(Platform))]
#[diesel(table_name = package_versions_platforms)]
pub struct PackageVersionPlatform {
    pub id: i32,
    pub package_version_id: i32,
    pub platform_id: i32,
}
