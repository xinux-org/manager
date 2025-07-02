use crate::schema::*;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = nixpkgs_sources)]
pub struct NixpkgsSource {
    pub id: i32,
    pub channel: String,
    pub git_ref: String,
    pub processed: bool,
}

#[derive(Insertable)]
#[diesel(table_name = nixpkgs_sources)]
pub struct NewNixpkgSource<'a> {
    pub channel: &'a str,
    pub git_ref: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = nixpkgs_sources)]
pub struct ProcessedNixpkgSource {
    pub processed: bool,
}

impl NixpkgsSource {
    pub fn find_by_channel(conn: &mut PgConnection, channel: &str) -> QueryResult<Self> {
        use crate::schema::nixpkgs_sources::dsl;

        dsl::nixpkgs_sources
            .filter(dsl::channel.eq(channel))
            .limit(1)
            .select(Self::as_select())
            .first(conn)
    }

    pub fn find_by_channel_and_ref(
        conn: &mut PgConnection,
        channel: &str,
        git_ref: &str,
    ) -> QueryResult<Self> {
        use crate::schema::nixpkgs_sources::dsl;

        dsl::nixpkgs_sources
            .filter(dsl::channel.eq(channel).and(dsl::git_ref.eq(git_ref)))
            .limit(1)
            .select(Self::as_select())
            .first(conn)
    }

    pub fn create(conn: &mut PgConnection, channel: &str, git_ref: &str) -> QueryResult<Self> {
        let new_row = NewNixpkgSource { channel, git_ref };

        diesel::insert_into(nixpkgs_sources::table)
            .values(&new_row)
            .returning(Self::as_returning())
            .get_result(conn)
    }

    pub fn update_processed(
        self: &Self,
        conn: &mut PgConnection,
        processed: bool,
    ) -> QueryResult<()> {
        use crate::schema::nixpkgs_sources::dsl;

        diesel::update(dsl::nixpkgs_sources)
            .filter(dsl::id.eq(self.id))
            .set(dsl::processed.eq(processed))
            .execute(conn)
            .map(|_| ())
    }
}
