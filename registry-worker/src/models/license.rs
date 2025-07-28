use crate::{
    libs::super_orm::{CreateAll, FindAll, FindOrCreateAll, WithOutput},
    schema::*,
    types::{AsyncPool, ProcessError, ProcessResult}
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = licenses)]
pub struct License {
    pub id: i32,
    pub fullname: String,
    pub url: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = licenses)]
pub struct NewLicense {
    pub fullname: String,
    pub url: Option<String>,
}

impl NewLicense {
    pub fn from_values(fullname: String, url: Option<String>) -> Self {
        Self { fullname, url }
    }
}

impl WithOutput for NewLicense {
    type Output = License;

    fn is_same(&self, other: &Self::Output) -> bool {
        self.fullname.eq(&other.fullname) &&
        self.url.eq(&other.url)
    }
}


impl FindAll for NewLicense {
    async fn find_all(pool: AsyncPool, new: &Vec<&Self>) -> ProcessResult<Vec<Self::Output>> {
        use crate::schema::licenses::dsl;
        let mut conn = pool.get().await?;
        let names: Vec<_> = new.iter().map(|new| &new.fullname).collect();
        let urls: Vec<_> = new.iter().filter_map(|new| new.url.as_deref()).collect();
       
        dsl::licenses
            .filter(dsl::fullname.eq_any(names))
            .or_filter(dsl::url.eq_any(urls))
            .select(Self::Output::as_select())
            .load(&mut conn)
            .await
            .map_err(ProcessError::DieselError)
    }
}

impl CreateAll for NewLicense {
    async fn create_all(pool: AsyncPool, new: &Vec<Self>) -> ProcessResult<Vec<Self::Output>> {
        let mut conn = pool.get().await?;

        diesel::insert_into(licenses::table)
            .values(new)
            .returning(Self::Output::as_returning())
            .on_conflict_do_nothing()
            .get_results(&mut conn)
            .await
            .map_err(ProcessError::DieselError)
    }
}

impl FindOrCreateAll for NewLicense {}
