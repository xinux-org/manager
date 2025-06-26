use crate::{
    models::{maintainer::Maintainer, package_version::PackageVersion},
    schema::*,
};
use diesel::prelude::*;

#[derive(Queryable, Selectable, Associations, Identifiable, Debug, PartialEq)]
#[diesel(belongs_to(PackageVersion))]
#[diesel(belongs_to(Maintainer))]
#[diesel(table_name = package_versions_maintainers)]
pub struct PackageVersionMaintainer {
    pub id: i32,
    pub package_version_id: i32,
    pub maintainer_id: i32,
}
