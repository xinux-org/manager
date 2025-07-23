use crate::{
    schema::*,
    types::{AsyncPool, ProcessError, ProcessResult},
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use flake_info::data::export::License as FlakeLicense;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = licenses)]
pub struct License {
    pub id: i32,
    pub fullname: String,
    pub url: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = licenses)]
pub struct NewLicense<'a> {
    pub fullname: &'a str,
    pub url: Option<&'a str>,
}

impl License {
    pub async fn find_by_name(pool: AsyncPool, license: FlakeLicense) -> ProcessResult<Self> {
        use crate::schema::licenses::dsl;

        let mut conn = pool.get().await?;

        dsl::licenses
            .filter(dsl::fullname.eq(&license.fullName))
            .limit(1)
            .select(Self::as_select())
            .first(&mut conn)
            .await
            .map_err(ProcessError::DieselError)
    }

    pub async fn create(pool: AsyncPool, license: FlakeLicense) -> ProcessResult<Self> {
        let mut conn = pool.get().await?;
        let new_row = NewLicense {
            fullname: &license.fullName,
            url: license.url.as_deref(),
        };

        diesel::insert_into(licenses::table)
            .values(&new_row)
            .returning(Self::as_returning())
            .get_result(&mut conn)
            .await
            .map_err(ProcessError::DieselError)
    }
}
