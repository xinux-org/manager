use crate::{models::source::Source, schema::*};
use diesel::prelude::*;

#[derive(Queryable, Selectable, Associations, Identifiable, Debug, PartialEq)]
#[diesel(belongs_to(Source))]
#[diesel(table_name = revisions)]
pub struct Revision {
    pub id: i32,
    pub source_id: i32,
}
