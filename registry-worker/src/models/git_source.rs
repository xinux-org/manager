use crate::schema::*;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = git_sources)]
pub struct GitSource {
    pub id: i32,
    pub url: String,
}
