-- Your SQL goes here

ALTER TABLE "package_versions" ADD CONSTRAINT "package_versions_unique_package_version" UNIQUE ("package_id", "version");
