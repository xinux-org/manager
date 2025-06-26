use crate::schema::*;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = licenses)]
pub struct License {
    pub id: i32,
    pub name: String,
    pub license: Option<String>,
    pub fullname: Option<String>,
    pub shortname: Option<String>,
    pub url: Option<String>,
}
