-- Your SQL goes here





ALTER TABLE "package_versions" DROP COLUMN "source_id";
ALTER TABLE "package_versions" ADD COLUMN "nixpkgs_source_id" INT4;
ALTER TABLE "package_versions" ADD COLUMN "git_source_id" INT4;
ALTER TABLE "package_versions" ADD COLUMN "git_host_source_id" INT4;





DROP TABLE IF EXISTS "sources";
