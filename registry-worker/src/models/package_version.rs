use crate::{
    models::{license::License, package::Package, revision::Revision},
    schema::*,
};
use diesel::prelude::*;

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
