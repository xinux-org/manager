use crate::{
    models::NewPackageVersionSource,
    types::{AsyncPool, ProcessResult},
};

pub trait Source {
    fn is_processed(&self) -> bool;
    fn set_processed(
        &self,
        pool: AsyncPool,
        processed: bool,
    ) -> impl std::future::Future<Output = ProcessResult<()>> + Send;
    fn update_package_version_source_id(
        &self,
        package_version_source: &mut NewPackageVersionSource,
    );
}
