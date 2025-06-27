use crate::schema::*;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = packages)]
pub struct Package {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub homepage: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = packages)]
pub struct NewPackage<'a> {
    pub name: &'a str,
    pub description: Option<String>,
    pub homepage: Option<String>,
}

impl Package {
    pub fn find_by_name(conn: &mut PgConnection, name: &str) -> QueryResult<Self> {
        use crate::schema::packages::dsl;

        dsl::packages
            .filter(dsl::name.eq(name))
            .limit(1)
            .select(Self::as_select())
            .first(conn)
    }

    pub fn create(
        conn: &mut PgConnection,
        name: &str,
        description: Option<String>,
        homepage: Option<String>,
    ) -> QueryResult<Self> {
        let new_row = NewPackage {
            name,
            description,
            homepage,
        };

        diesel::insert_into(packages::table)
            .values(&new_row)
            .returning(Self::as_returning())
            .get_result(conn)
    }
}
