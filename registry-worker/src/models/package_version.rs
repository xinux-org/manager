use crate::{
    models::{license::License, package::Package},
    schema::*,
};
use diesel::prelude::*;
use flake_info::data::Derivation;

#[derive(Queryable, Identifiable, Selectable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Package))]
#[diesel(belongs_to(License))]
#[diesel(table_name = package_versions)]
pub struct PackageVersion {
    pub id: i32,
    pub package_id: i32,
    pub license_id: Option<i32>,
    pub available: bool,
    pub broken: bool,
    pub insecure: bool,
    pub changelog: Option<String>,
    pub version: String,
}

#[derive(Insertable, Default)]
#[diesel(table_name = package_versions)]
pub struct NewPackageVersion<'a> {
    pub package_id: i32,
    pub license_id: Option<i32>,
    pub available: bool,
    pub broken: bool,
    pub insecure: bool,
    pub changelog: Option<&'a str>,
    pub version: &'a str,
}

impl PackageVersion {
    pub fn find_by_package_and_version(
        conn: &mut PgConnection,
        package_id: i32,
        version: &str,
    ) -> QueryResult<Self> {
        use crate::schema::package_versions::dsl;

        dsl::package_versions
            .filter(dsl::package_id.eq(package_id).and(dsl::version.eq(version)))
            .limit(1)
            .select(Self::as_select())
            .first(conn)
    }

    pub fn create_from(
        conn: &mut PgConnection,
        package: &Package,
        derivation: &Derivation,
    ) -> QueryResult<Self> {
        if let Derivation::Package {
            package_pversion, ..
        } = derivation
        {
            let new_row = NewPackageVersion {
                package_id: package.id,
                version: package_pversion,
                ..Default::default()
            };

            diesel::insert_into(package_versions::table)
                .values(&new_row)
                .returning(Self::as_returning())
                .get_result(conn)
        } else {
            unreachable!("package version derivation must be a package!")
        }
    }
}
