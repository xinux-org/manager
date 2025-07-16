use crate::{
    models::{license::License, package::Package},
    schema::*,
    types::{AsyncPool, ProcessError, ProcessResult},
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

#[derive(Queryable, Identifiable, Selectable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Package))]
#[diesel(belongs_to(License))]
#[diesel(table_name = package_versions)]
pub struct PackageVersion {
    pub id: i32,
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
    pub package_id: i32,
    pub license_id: Option<i32>,
    pub available: bool,
    pub broken: bool,
    pub insecure: bool,
    pub changelog: Option<&'a str>,
    pub version: &'a str,
}

impl PackageVersion {
    pub async fn find_by_package_and_version(
        pool: AsyncPool,
        package_id: i32,
        version: &str,
    ) -> ProcessResult<Self> {
        use crate::schema::package_versions::dsl;
        let mut conn = pool.get().await?;

        dsl::package_versions
            .filter(dsl::package_id.eq(package_id).and(dsl::version.eq(version)))
            .limit(1)
            .select(Self::as_select())
            .first(&mut conn)
            .await
            .map_err(ProcessError::DieselError)
    }

    pub async fn create_from(
        pool: AsyncPool,
        package_id: i32,
        version: &String,
    ) -> ProcessResult<Self> {
        let mut conn = pool.get().await?;
        let new_row = NewPackageVersion {
            package_id,
            version,
            ..Default::default()
        };

        diesel::insert_into(package_versions::table)
            .values(&new_row)
            .returning(Self::as_returning())
            .get_result(&mut conn)
            .await
            .map_err(ProcessError::DieselError)
    }
}
