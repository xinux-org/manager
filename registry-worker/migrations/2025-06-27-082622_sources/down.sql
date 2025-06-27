-- This file should undo anything in `up.sql`


ALTER TABLE "package_versions" DROP COLUMN "source_id";
ALTER TABLE "package_versions" ADD COLUMN "revision_id" INT4 NOT NULL;





CREATE TABLE "revisions"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"source_id" INT4 NOT NULL
);

ALTER TABLE "sources" DROP COLUMN "nixpkgs_source_id";
ALTER TABLE "sources" DROP COLUMN "git_host_source_id";
ALTER TABLE "sources" DROP COLUMN "git_source_id";

DROP TABLE IF EXISTS "git_sources";
DROP TABLE IF EXISTS "git_host_sources";
DROP TABLE IF EXISTS "nixpkgs_sources";
