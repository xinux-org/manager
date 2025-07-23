use crate::{
    libs::super_orm::{Create, Find, FindOrCreate, WithOutput},
    schema::*,
    types::{AsyncPool, ProcessError, ProcessResult},
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq, Clone)]
#[diesel(table_name = packages)]
pub struct Package {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub homepage: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = packages)]
pub struct NewPackage {
    pub name: String,
    pub description: Option<String>,
    pub homepage: Option<String>,
}

impl NewPackage {
    pub fn from_values(
        name: String,
        description: Option<String>,
        homepage: Option<String>,
    ) -> Self {
        Self {
            name,
            description,
            homepage,
        }
    }
}

impl WithOutput for NewPackage {
    type Output = Package;

    fn is_same(&self, other: &Self::Output) -> bool {
        self.name.eq(&other.name)
    }
}

impl Find for NewPackage {
    async fn find_db(pool: AsyncPool, new: &Self) -> ProcessResult<Self::Output> {
        use crate::schema::packages::dsl;
        let mut conn = pool.get().await?;

        dsl::packages
            .filter(dsl::name.eq(&new.name))
            .limit(1)
            .select(Self::Output::as_select())
            .first(&mut conn)
            .await
            .map_err(ProcessError::DieselError)
    }
}

impl Create for NewPackage {
    async fn create_db(pool: AsyncPool, new: &Self) -> ProcessResult<Self::Output> {
        let mut conn = pool.get().await?;

        diesel::insert_into(packages::table)
            .values(new)
            .returning(Self::Output::as_returning())
            .get_result(&mut conn)
            .await
            .map_err(ProcessError::DieselError)
    }
}

impl FindOrCreate for NewPackage {}
