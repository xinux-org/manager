// @generated automatically by Diesel CLI.

diesel::table! {
    git_host_sources (id) {
        id -> Int4,
        host -> Varchar,
        owner -> Varchar,
        repo -> Varchar,
        git_ref -> Varchar,
    }
}

diesel::table! {
    git_sources (id) {
        id -> Int4,
        url -> Varchar,
    }
}

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
    nixpkgs_sources (id) {
        id -> Int4,
        channel -> Varchar,
        git_ref -> Varchar,
    }
}

diesel::table! {
    package_versions (id) {
        id -> Int4,
        package_id -> Int4,
        available -> Bool,
        broken -> Bool,
        insecure -> Bool,
        version -> Varchar,
        nixpkgs_source_id -> Nullable<Int4>,
        git_source_id -> Nullable<Int4>,
        git_host_source_id -> Nullable<Int4>,
        license_id -> Nullable<Int4>,
        changelog -> Nullable<Varchar>,
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
        name -> Varchar,
        description -> Nullable<Varchar>,
        homepage -> Nullable<Varchar>,
    }
}

diesel::table! {
    platforms (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::joinable!(package_versions -> packages (package_id));
diesel::joinable!(package_versions_maintainers -> maintainers (maintainer_id));
diesel::joinable!(package_versions_maintainers -> package_versions (package_version_id));
diesel::joinable!(package_versions_platforms -> package_versions (package_version_id));
diesel::joinable!(package_versions_platforms -> platforms (platform_id));

diesel::allow_tables_to_appear_in_same_query!(
    git_host_sources,
    git_sources,
    licenses,
    maintainers,
    nixpkgs_sources,
    package_versions,
    package_versions_maintainers,
    package_versions_platforms,
    packages,
    platforms,
);
