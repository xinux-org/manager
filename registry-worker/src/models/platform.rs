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

#[derive(Insertable, Debug, Clone)]
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

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::libs::{
        diesel_test_mock::diesel_test_pool,
        super_orm::{CreateAll, FindAll, FindOrCreateAll},
    };

    use super::NewPlatform;

    #[tokio::test]
    async fn it_works() {
        let pool = diesel_test_pool().await;

        let new_x86_64_linux = NewPlatform::from_values("x86_64-linux".to_string());

        assert_eq!(
            NewPlatform::find_all(pool.clone(), &vec![&new_x86_64_linux])
                .await
                .expect("Should retrieve data form database")
                .len(),
            0,
            "initially, there is no platforms"
        );

        let result = NewPlatform::find_or_create_all(pool.clone(), vec![new_x86_64_linux.clone()])
            .await
            .expect("Should be able to create platform");

        assert_eq!(result.len(), 1, "should create one, because it is empty");

        let result = NewPlatform::find_all(pool.clone(), &vec![&new_x86_64_linux])
            .await
            .expect("Should retrieve data form database");

        assert_eq!(
            result.len(),
            1,
            "now we created a platform and there should be one"
        );

        let new_aarch64_linux = NewPlatform::from_values("aarch64-linux".to_string());

        let result = NewPlatform::find_all(pool.clone(), &vec![&new_aarch64_linux])
            .await
            .expect("Should be able to create platform");

        assert_eq!(
            result.len(),
            0,
            "aarch64-linux is not inserted yet, therefore this should be empty"
        );

        let result = NewPlatform::find_or_create_all(
            pool.clone(),
            vec![new_x86_64_linux.clone(), new_aarch64_linux.clone()],
        )
        .await
        .expect("Should be able to create platform");

        assert_eq!(
            result.len(),
            2,
            "this should create aarch64-linux, and get x86_64-linux"
        );

        let first = result.iter().find(|p| p.name == new_x86_64_linux.name);
        assert!(first.is_some(), "there should be x86_64-linux");

        let first = first.unwrap();
        assert_eq!(first.id, 1, "should find x86_64-linux's id");

        let result =
            NewPlatform::create_all(pool.clone(), &vec![new_x86_64_linux, new_aarch64_linux])
                .await
                .expect("Should be able to create");

        assert_eq!(
            result.len(),
            0,
            "should should not create nothing, because there is already created ones"
        );
    }
}
