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

#[derive(Insertable)]
#[diesel(table_name = package_versions_platforms)]
pub struct NewPackageVersionPlatform {
    pub package_version_id: i32,
    pub platform_id: i32,
}

impl PackageVersionPlatform {
    pub fn create_all_only(
        conn: &mut PgConnection,
        package_version: &PackageVersion,
        platforms: &[Platform],
    ) -> QueryResult<()> {
        diesel::insert_into(package_versions_platforms::table)
            .values(
                platforms
                    .iter()
                    .map(|platform| NewPackageVersionPlatform {
                        platform_id: platform.id,
                        package_version_id: package_version.id,
                    })
                    .collect::<Vec<_>>(),
            )
            .execute(conn)
            .map(|_| ())
    }
}
