use crate::{
    libs::super_orm::{CreateAll, FindAll, FindOrCreateAll, WithOutput},
    schema::*,
    types::{AsyncPool, ProcessError, ProcessResult}
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = maintainers)]
pub struct Maintainer {
    pub id: i32,
    pub name: Option<String>,
    pub github: Option<String>,
    pub email: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = maintainers)]
pub struct NewMaintainer {
    pub name: Option<String>,
    pub github: Option<String>,
    pub email: Option<String>,
}

impl NewMaintainer {
    pub fn from_values(name: Option<String>, github: Option<String>, email: Option<String>) -> Self {
        Self { name, github, email }
    }
}

impl WithOutput for NewMaintainer {
    type Output = Maintainer;
    fn is_same(&self, other: &Self::Output) -> bool {
        self.name.eq(&other.name) &&
        self.github.eq(&other.github) &&
        self.email.eq(&other.email)
    }
}

impl FindAll for NewMaintainer {
    async fn find_all(pool: AsyncPool, new: &Vec<&Self>) -> ProcessResult<Vec<Self::Output>> {
        use crate::schema::maintainers::dsl;
        let mut conn = pool.get().await?;
        let names: Vec<_> = new.iter().filter_map(|new| new.name.as_deref()).collect();
        let githubs: Vec<_> = new.iter().filter_map(|new| new.github.as_deref()).collect();
        let emails: Vec<_> = new.iter().filter_map(|new| new.email.as_deref()).collect();

        dsl::maintainers
            .filter(dsl::name.eq_any(names))
            .or_filter(dsl::email.eq_any(emails))
            .or_filter(dsl::github.eq_any(githubs))
            .select(Self::Output::as_select())
            .load(&mut conn)
            .await
            .map_err(ProcessError::DieselError)
    }
}

impl CreateAll for NewMaintainer {
    async fn create_all(pool: AsyncPool, new: &Vec<Self>) -> ProcessResult<Vec<Self::Output>> {
        let mut conn = pool.get().await?;

        diesel::insert_into(maintainers::table)
            .values(new)
            .returning(Self::Output::as_returning())
            .on_conflict_do_nothing()
            .get_results(&mut conn)
            .await
            .map_err(ProcessError::DieselError)
    }
}

impl FindOrCreateAll for NewMaintainer {}
