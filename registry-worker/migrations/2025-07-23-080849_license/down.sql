-- This file should undo anything in `up.sql`


ALTER TABLE "licenses" DROP COLUMN "fullname";
ALTER TABLE "licenses" ADD COLUMN "name" VARCHAR NOT NULL DEFAULT '';
ALTER TABLE "licenses" ADD COLUMN "license" VARCHAR;
ALTER TABLE "licenses" ADD COLUMN "fullname" VARCHAR;











