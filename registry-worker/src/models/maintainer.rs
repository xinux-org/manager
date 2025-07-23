use crate::{
    schema::*,
    types::{AsyncPool, ProcessError, ProcessResult},
};
use diesel::{prelude::*, result::Error};
use diesel_async::RunQueryDsl;
use flake_info::data::export::{Maintainer as FlakeMaintainer};

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
pub struct NewMaintainer<'a> {
    pub name: Option<&'a str>,
    pub github: Option<&'a str>,
    pub email: Option<&'a str>,
}

impl Maintainer {
    pub async fn find_by_maintainer(
        pool: AsyncPool,
        maintainer: FlakeMaintainer
    ) -> ProcessResult<Self> {
        use crate::schema::maintainers::dsl;
        let mut conn = pool.get().await?;

        if let Some(email) = &maintainer.email {
            return dsl::maintainers
                .filter(dsl::email.eq(email))
                .limit(1)
                .select(Self::as_select())
                .first(&mut conn)
                .await
                .map_err(ProcessError::DieselError);
        } else if let Some(github) = &maintainer.github {
            return dsl::maintainers
                .filter(dsl::github.eq(github))
                .limit(1)
                .select(Self::as_select())
                .first(&mut conn)
                .await
                .map_err(ProcessError::DieselError);
        } else if let Some(name) = &maintainer.name {
            return dsl::maintainers
                .filter(dsl::name.eq(name))
                .limit(1)
                .select(Self::as_select())
                .first(&mut conn)
                .await
                .map_err(ProcessError::DieselError);
        } else {
            Err(ProcessError::DieselError(Error::NotFound))
        }
    }

    pub async fn create(
        pool: AsyncPool,
        maintainer: FlakeMaintainer
    ) -> ProcessResult<Self> {
        let mut conn = pool.get().await?;
        let new_row = NewMaintainer {
            name: maintainer.name.as_deref(),
            github: maintainer.github.as_deref(),
            email: maintainer.email.as_deref(),
        };

        diesel::insert_into(maintainers::table)
            .values(&new_row)
            .returning(Self::as_returning())
            .get_result(&mut conn)
            .await
            .map_err(ProcessError::DieselError)
    }
}
