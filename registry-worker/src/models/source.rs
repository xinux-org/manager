use super::{GitHostSource, GitSource, NixpkgsSource};

pub enum Source {
    Nixpkgs(NixpkgsSource),
    GitHost(GitHostSource),
    Git(GitSource),
}
