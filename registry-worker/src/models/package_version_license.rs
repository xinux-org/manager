use crate::{
    models::{license::License, package_version::PackageVersion},
    schema::*,
    types::{AsyncPool, ProcessError, ProcessResult},
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

#[derive(Queryable, Selectable, Associations, Identifiable, Debug, PartialEq)]
#[diesel(belongs_to(PackageVersion))]
#[diesel(belongs_to(License))]
#[diesel(table_name = package_versions_licenses)]
pub struct PackageVersionLicense {
    pub id: i32,
    pub package_version_id: i32,
    pub license_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = package_versions_licenses)]
pub struct NewPackageVersionLicense {
    pub package_version_id: i32,
    pub license_id: i32,
}

impl PackageVersionLicense {
    pub async fn create_all_only(
        pool: AsyncPool,
        package_version: &PackageVersion,
        licenses: &[License],
    ) -> ProcessResult<()> {
        let mut conn = pool.get().await?;
        diesel::insert_into(package_versions_licenses::table)
            .values(
                licenses
                    .iter()
                    .map(|licence| NewPackageVersionLicense {
                        license_id: licence.id,
                        package_version_id: package_version.id,
                    })
                    .collect::<Vec<_>>(),
            )
            .execute(&mut conn)
            .await
            .map(|_| ())
            .map_err(ProcessError::DieselError)
    }
}
