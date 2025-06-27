-- This file should undo anything in `up.sql`





ALTER TABLE "package_versions" DROP COLUMN "license_id";
ALTER TABLE "package_versions" DROP COLUMN "changelog";
ALTER TABLE "package_versions" ADD COLUMN "license_id" INT4 NOT NULL;
ALTER TABLE "package_versions" ADD COLUMN "changelog" VARCHAR NOT NULL;





