use bb8::{Pool, RunError};
use diesel::result::Error as DieselError;
use diesel_async::{
    AsyncPgConnection,
    pooled_connection::{AsyncDieselConnectionManager, PoolError},
};
use octocrab::Error as OctocrabError;

pub type AsyncPool = Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;
pub type ProcessResult<T> = Result<T, ProcessError>;

#[derive(Debug)]
pub enum ProcessError {
    PgPool(RunError<PoolError>),
    DieselError(DieselError),
    OctocrabError(OctocrabError),
    SourceCreateFailed,
    NotImplemented,
}

impl From<RunError<PoolError>> for ProcessError {
    fn from(value: RunError<PoolError>) -> Self {
        Self::PgPool(value)
    }
}
