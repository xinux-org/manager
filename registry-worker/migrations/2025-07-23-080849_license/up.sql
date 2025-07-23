-- Your SQL goes here


ALTER TABLE "licenses" DROP COLUMN "name";
ALTER TABLE "licenses" DROP COLUMN "license";
ALTER TABLE "licenses" DROP COLUMN "fullname";
ALTER TABLE "licenses" ADD COLUMN "fullname" VARCHAR NOT NULL DEFAULT '';











