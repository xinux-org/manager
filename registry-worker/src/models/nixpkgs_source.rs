use crate::schema::*;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = nixpkgs_sources)]
pub struct NixpkgsSource {
    pub id: i32,
    pub channel: String,
    pub git_ref: String,
}
