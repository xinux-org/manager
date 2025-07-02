-- This file should undo anything in `up.sql`





ALTER TABLE "package_versions" ADD COLUMN "nixpkgs_source_id" INT4;
ALTER TABLE "package_versions" ADD COLUMN "git_source_id" INT4;
ALTER TABLE "package_versions" ADD COLUMN "git_host_source_id" INT4;





DROP TABLE IF EXISTS "package_versions_sources";
