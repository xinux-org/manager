use crate::schema::*;
use diesel::prelude::*;
use moka::sync::Cache;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq, Clone)]
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

impl Platform {
    pub fn find_by_name(conn: &mut PgConnection, name: &str) -> QueryResult<Self> {
        use crate::schema::platforms::dsl;

        dsl::platforms
            .filter(dsl::name.eq(name))
            .limit(1)
            .select(Self::as_select())
            .first(conn)
    }

    pub fn create(conn: &mut PgConnection, name: &str) -> QueryResult<Self> {
        let new_row = NewPlatform { name };

        diesel::insert_into(platforms::table)
            .values(&new_row)
            .returning(Self::as_returning())
            .get_result(conn)
    }

    pub fn cached_get_or_create(
        conn: &mut PgConnection,
        cache: &mut Cache<String, Self>,
        name: &str,
    ) -> QueryResult<Self> {
        cache
            .get(name)
            .ok_or(diesel::result::Error::NotFound)
            .or_else(|_| {
                Self::find_by_name(conn, name)
                    .or_else(|_| Self::create(conn, name))
                    .and_then(|platform| {
                        cache.insert(name.to_string(), platform);
                        cache.get(name).ok_or(diesel::result::Error::NotFound)
                    })
            })
    }
}
