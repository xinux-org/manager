-- Your SQL goes here








ALTER TABLE "packages" ADD COLUMN "name" VARCHAR NOT NULL;
ALTER TABLE "packages" DROP COLUMN "description";
ALTER TABLE "packages" DROP COLUMN "homepage";
ALTER TABLE "packages" ADD COLUMN "description" VARCHAR;
ALTER TABLE "packages" ADD COLUMN "homepage" VARCHAR;

ALTER TABLE "packages" ADD CONSTRAINT "packages_unique_name" UNIQUE ("name");
