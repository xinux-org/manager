use crate::{
    libs::super_orm::{CreateAll, WithOutput},
    models::{package_version::PackageVersion, platform::Platform},
    schema::*,
    types::{AsyncPool, ProcessError, ProcessResult},
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

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

impl NewPackageVersionPlatform {
    pub fn from_values(package_version_id: i32, platform_id: i32) -> Self {
        Self {
            package_version_id,
            platform_id,
        }
    }
}

impl WithOutput for NewPackageVersionPlatform {
    type Output = ();

    fn is_same(&self, _: &Self::Output) -> bool {
        false
    }
}

impl CreateAll for NewPackageVersionPlatform {
    async fn create_all(pool: AsyncPool, new: &Vec<Self>) -> ProcessResult<Vec<Self::Output>> {
        let mut conn = pool.get().await?;
        diesel::insert_into(package_versions_platforms::table)
            .values(new)
            .execute(&mut conn)
            .await
            .map(|_| vec![])
            .map_err(ProcessError::DieselError)
    }
}
