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

pub fn find_platform_by_name(conn: &mut PgConnection, name: &str) -> Option<Platform> {
    use crate::schema::platforms::dsl;

    dsl::platforms
        .filter(dsl::name.eq(name))
        .limit(1)
        .select(Platform::as_select())
        .load(conn)
        .ok()
        .and_then(|mut platforms| platforms.pop())
}

pub fn create_platform(conn: &mut PgConnection, name: &str) -> Option<Platform> {
    let new_platform = NewPlatform { name };

    diesel::insert_into(platforms::table)
        .values(&new_platform)
        .returning(Platform::as_returning())
        .get_result(conn)
        .ok()
}
