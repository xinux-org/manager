use crate::types::{AsyncPool, ProcessResult};

use super::{GitHostSource, GitSource, NixpkgsSource};

#[derive(Clone, Debug)]
pub enum Source {
    Nixpkgs(NixpkgsSource),
    GitHost(GitHostSource),
    Git(GitSource),
}
impl Source {
    pub fn is_processed(&self) -> bool {
        match self {
            Self::Nixpkgs(nixpkgs) => nixpkgs.processed,
            _ => false,
        }
    }

    pub async fn set_processed(self, pool: AsyncPool, processed: bool) -> ProcessResult<()> {
        match self {
            Self::Nixpkgs(nixpkgs) => nixpkgs.update_processed(pool, processed).await,
            _ => Ok(()),
        }
    }
}
