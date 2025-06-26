use crate::schema::*;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = platforms)]
pub struct Platform {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = platforms)]
pub struct NewPlatform<'a> {
    pub name: &'a str,
}
