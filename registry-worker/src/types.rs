use bb8::{Pool, RunError};
use diesel::result::Error;
use diesel_async::{
    AsyncPgConnection,
    pooled_connection::{AsyncDieselConnectionManager, PoolError},
};

pub type AsyncPool = Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;
pub type ProcessResult<T> = Result<T, ProcessError>;

#[derive(Debug)]
pub enum ProcessError {
    PgPool(RunError<PoolError>),
    DieselError(Error),
    SourceCreateFailed,
    NotImplemented,
}

impl From<RunError<PoolError>> for ProcessError {
    fn from(value: RunError<PoolError>) -> Self {
        Self::PgPool(value)
    }
}

impl From<Error> for ProcessError {
    fn from(value: Error) -> Self {
        Self::DieselError(value)
    }
}
