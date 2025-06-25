// @generated automatically by Diesel CLI.

diesel::table! {
    licenses (id) {
        id -> Int4,
        name -> Varchar,
        license -> Nullable<Varchar>,
        fullname -> Nullable<Varchar>,
        shortname -> Nullable<Varchar>,
        url -> Nullable<Varchar>,
    }
}

diesel::table! {
    maintainers (id) {
        id -> Int4,
        name -> Varchar,
        github -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
    }
}

diesel::table! {
    package_versions (id) {
        id -> Int4,
        revision_id -> Int4,
        package_id -> Int4,
        license_id -> Int4,
        available -> Bool,
        broken -> Bool,
        insecure -> Bool,
        changelog -> Varchar,
        version -> Varchar,
    }
}

diesel::table! {
    package_versions_maintainers (id) {
        id -> Int4,
        package_version_id -> Int4,
        maintainer_id -> Int4,
    }
}

diesel::table! {
    package_versions_platforms (id) {
        id -> Int4,
        package_version_id -> Int4,
        platform_id -> Int4,
    }
}

diesel::table! {
    packages (id) {
        id -> Int4,
        description -> Varchar,
        homepage -> Varchar,
    }
}

diesel::table! {
    platforms (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    revisions (id) {
        id -> Int4,
        source_id -> Int4,
    }
}

diesel::table! {
    sources (id) {
        id -> Int4,
    }
}

diesel::joinable!(package_versions -> licenses (license_id));
diesel::joinable!(package_versions -> packages (package_id));
diesel::joinable!(package_versions -> revisions (revision_id));
diesel::joinable!(package_versions_maintainers -> maintainers (maintainer_id));
diesel::joinable!(package_versions_maintainers -> package_versions (package_version_id));
diesel::joinable!(package_versions_platforms -> package_versions (package_version_id));
diesel::joinable!(package_versions_platforms -> platforms (platform_id));

diesel::allow_tables_to_appear_in_same_query!(
    licenses,
    maintainers,
    package_versions,
    package_versions_maintainers,
    package_versions_platforms,
    packages,
    platforms,
    revisions,
    sources,
);
