use crate::{
    libs::super_orm::{CreateAll, WithOutput}, models::{maintainer::Maintainer, package_version::PackageVersion}, schema::*, types::{AsyncPool, ProcessError, ProcessResult}
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

#[derive(Queryable, Selectable, Associations, Identifiable, Debug, PartialEq)]
#[diesel(belongs_to(PackageVersion))]
#[diesel(belongs_to(Maintainer))]
#[diesel(table_name = package_versions_maintainers)]
pub struct PackageVersionMaintainer {
    pub id: i32,
    pub package_version_id: i32,
    pub maintainer_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = package_versions_maintainers)]
pub struct NewPackageVersionMaintainer {
    pub package_version_id: i32,
    pub maintainer_id: i32,
}

impl NewPackageVersionMaintainer {
    pub fn from_values(package_version_id: i32, maintainer_id: i32) -> Self {
        Self { package_version_id, maintainer_id }
    }
}

impl WithOutput for NewPackageVersionMaintainer {
    type Output = ();

    fn is_same(&self, _: &Self::Output) -> bool {
        false
    }
}

impl CreateAll for NewPackageVersionMaintainer {
    async fn create_all(pool: AsyncPool, new: &Vec<Self>) -> ProcessResult<Vec<Self::Output>> {
        let mut conn = pool.get().await?;
        diesel::insert_into(package_versions_maintainers::table)
            .values(new)
            .execute(&mut conn)
            .await
            .map(|_| vec![])
            .map_err(ProcessError::DieselError)
    }
}
