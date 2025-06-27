-- Your SQL goes here





ALTER TABLE "package_versions" DROP COLUMN "license_id";
ALTER TABLE "package_versions" DROP COLUMN "changelog";
ALTER TABLE "package_versions" ADD COLUMN "license_id" INT4;
ALTER TABLE "package_versions" ADD COLUMN "changelog" VARCHAR;





