// @generated automatically by Diesel CLI.

diesel::table! {
    git_host_sources (id) {
        id -> Int4,
        host -> Varchar,
        owner -> Varchar,
        repo -> Varchar,
        git_ref -> Varchar,
        processed -> Bool,
        created_at -> Timestamp,
        locked_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    git_sources (id) {
        id -> Int4,
        url -> Varchar,
        processed -> Bool,
        created_at -> Timestamp,
        locked_at -> Nullable<Timestamp>,
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
        created_at -> Timestamp,
    }
}

diesel::table! {
    maintainers (id) {
        id -> Int4,
        name -> Varchar,
        github -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    nixpkgs_channels (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    nixpkgs_channels_sources (id) {
        id -> Int4,
        channel_id -> Int4,
        source_id -> Int4,
    }
}

diesel::table! {
    nixpkgs_sources (id) {
        id -> Int4,
        processed -> Bool,
        committed_at -> Timestamp,
        created_at -> Timestamp,
        locked_at -> Nullable<Timestamp>,
        sha -> Varchar,
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
        license_id -> Nullable<Int4>,
        changelog -> Nullable<Varchar>,
        nixpkgs_source_id -> Nullable<Int4>,
        git_source_id -> Nullable<Int4>,
        git_host_source_id -> Nullable<Int4>,
        created_at -> Timestamp,
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
    package_versions_sources (id) {
        id -> Int4,
        package_version_id -> Int4,
        nixpkgs_source_id -> Nullable<Int4>,
        git_source_id -> Nullable<Int4>,
        git_host_source_id -> Nullable<Int4>,
    }
}

diesel::table! {
    packages (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        homepage -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    platforms (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::joinable!(nixpkgs_channels_sources -> nixpkgs_channels (channel_id));
diesel::joinable!(nixpkgs_channels_sources -> nixpkgs_sources (source_id));
diesel::joinable!(package_versions -> packages (package_id));
diesel::joinable!(package_versions_maintainers -> maintainers (maintainer_id));
diesel::joinable!(package_versions_maintainers -> package_versions (package_version_id));
diesel::joinable!(package_versions_platforms -> package_versions (package_version_id));
diesel::joinable!(package_versions_platforms -> platforms (platform_id));
diesel::joinable!(package_versions_sources -> git_host_sources (git_host_source_id));
diesel::joinable!(package_versions_sources -> git_sources (git_source_id));
diesel::joinable!(package_versions_sources -> nixpkgs_sources (nixpkgs_source_id));
diesel::joinable!(package_versions_sources -> package_versions (package_version_id));

diesel::allow_tables_to_appear_in_same_query!(
    git_host_sources,
    git_sources,
    licenses,
    maintainers,
    nixpkgs_channels,
    nixpkgs_channels_sources,
    nixpkgs_sources,
    package_versions,
    package_versions_maintainers,
    package_versions_platforms,
    package_versions_sources,
    packages,
    platforms,
);
