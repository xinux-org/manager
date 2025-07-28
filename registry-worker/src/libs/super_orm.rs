use futures::future::Either;
use itertools::Itertools;

use crate::types::{AsyncPool, ProcessResult};

pub trait WithOutput: Sized {
    type Output;

    fn is_same(&self, other: &Self::Output) -> bool;
}

pub trait Find: WithOutput {
    async fn find_db(pool: AsyncPool, new: &Self) -> ProcessResult<Self::Output>;
    #[allow(dead_code)]
    async fn find(&self, pool: AsyncPool) -> ProcessResult<Self::Output> {
        Self::find_db(pool, self).await
    }
}

pub trait FindAll: WithOutput {
    async fn find_all(pool: AsyncPool, new: &Vec<&Self>) -> ProcessResult<Vec<Self::Output>>;
}

pub trait Create: WithOutput {
    async fn create_db(pool: AsyncPool, new: &Self) -> ProcessResult<Self::Output>;
    #[allow(dead_code)]
    async fn create(&self, pool: AsyncPool) -> ProcessResult<Self::Output> {
        Self::create_db(pool, self).await
    }
}

pub trait CreateAll: WithOutput {
    async fn create_all(pool: AsyncPool, new: &Vec<Self>) -> ProcessResult<Vec<Self::Output>>;
}

pub trait FindOrCreate: Find + Create {
    async fn find_or_create_db(pool: AsyncPool, new: &Self) -> ProcessResult<Self::Output> {
        Self::find_db(pool.clone(), new)
            .await
            .map_or_else(
                |_| Either::Left(async { Self::create_db(pool, new).await }),
                |v| Either::Right(async { Ok(v) }),
            )
            .await
    }
    async fn find_or_create(&self, pool: AsyncPool) -> ProcessResult<Self::Output> {
        Self::find_or_create_db(pool, self).await
    }
}

pub trait FindOrCreateAll: FindAll + CreateAll {
    async fn find_or_create_all(
        pool: AsyncPool,
        new: Vec<Self>,
    ) -> ProcessResult<Vec<Self::Output>> {
        Self::create_all(pool.clone(), &new)
            .await
            .map_or_else(
                |err| Either::Left(async { Err(err) }),
                |from_insert| {
                    Either::Right(async move {
                        let excluded = &new
                            .iter()
                            .filter(|row| {
                                from_insert
                                    .iter()
                                    .find_position(|inner_row| row.is_same(*inner_row))
                                    .is_none()
                            })
                            .collect();

                        Self::find_all(pool, excluded)
                            .await
                            .map(|from_select| vec![from_insert, from_select].into_iter().concat())
                    })
                },
            )
            .await
    }
}
