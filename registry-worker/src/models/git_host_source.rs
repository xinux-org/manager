use crate::schema::*;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = git_host_sources)]
pub struct GitHostSource {
    pub id: i32,
    pub host: String,
    pub owner: String,
    pub repo: String,
    pub git_ref: String,
}
