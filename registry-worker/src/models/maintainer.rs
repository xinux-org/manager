use crate::schema::*;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = maintainers)]
pub struct Maintainer {
    pub id: i32,
    pub name: String,
    pub github: Option<String>,
    pub email: Option<String>,
}
