use crate::{
    models::{GitHostSource, GitSource, NixpkgsSource},
    schema::*,
};
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(NixpkgsSource))]
#[diesel(belongs_to(GitHostSource))]
#[diesel(belongs_to(GitSource))]
#[diesel(table_name = sources)]
pub struct Source {
    pub id: i32,
    pub nixpkgs_source_id: Option<f32>,
    pub git_host_source_id: Option<f32>,
    pub git_source_id: Option<f32>,
}
