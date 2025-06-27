-- This file should undo anything in `up.sql`





ALTER TABLE "package_versions" DROP COLUMN "nixpkgs_source_id";
ALTER TABLE "package_versions" DROP COLUMN "git_source_id";
ALTER TABLE "package_versions" DROP COLUMN "git_host_source_id";
ALTER TABLE "package_versions" ADD COLUMN "source_id" INT4 NOT NULL;





CREATE TABLE "sources"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"nixpkgs_source_id" INT4,
	"git_host_source_id" INT4,
	"git_source_id" INT4
);

