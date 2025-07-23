use crate::{
    libs::super_orm::{CreateAll, FindAll, FindOrCreateAll, WithOutput},
    schema::*,
    types::{AsyncPool, ProcessError, ProcessResult},
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq, Clone)]
#[diesel(table_name = platforms)]
pub struct Platform {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = platforms)]
pub struct NewPlatform {
    pub name: String,
}

impl NewPlatform {
    pub fn from_values(name: String) -> Self {
        Self { name }
    }
}

impl WithOutput for NewPlatform {
    type Output = Platform;

    fn is_same(&self, other: &Self::Output) -> bool {
        self.name.eq(&other.name)
    }
}

impl FindAll for NewPlatform {
    async fn find_all(pool: AsyncPool, new: &Vec<&Self>) -> ProcessResult<Vec<Self::Output>> {
        use crate::schema::platforms::dsl;
        let mut conn = pool.get().await?;
        let names: Vec<_> = new.iter().map(|new| &new.name).collect();

        dsl::platforms
            .filter(dsl::name.eq_any(names))
            .select(Self::Output::as_select())
            .load(&mut conn)
            .await
            .map_err(ProcessError::DieselError)
    }
}

impl CreateAll for NewPlatform {
    async fn create_all(pool: AsyncPool, new: &Vec<Self>) -> ProcessResult<Vec<Self::Output>> {
        let mut conn = pool.get().await?;

        diesel::insert_into(platforms::table)
            .values(new)
            .returning(Self::Output::as_returning())
            .on_conflict_do_nothing()
            .get_results(&mut conn)
            .await
            .map_err(ProcessError::DieselError)
    }
}

impl FindOrCreateAll for NewPlatform {}
