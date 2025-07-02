use diesel::{PgConnection, QueryResult};

use super::{GitHostSource, GitSource, NixpkgsSource};

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

    pub fn set_processed(&self, conn: &mut PgConnection, processed: bool) -> QueryResult<()> {
        match self {
            Self::Nixpkgs(nixpkgs) => nixpkgs.update_processed(conn, processed),
            _ => Ok(()),
        }
    }
}
