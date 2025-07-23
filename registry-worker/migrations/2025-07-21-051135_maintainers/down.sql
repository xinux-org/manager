-- This file should undo anything in `up.sql`



ALTER TABLE "maintainers" DROP COLUMN "name";
ALTER TABLE "maintainers" ADD COLUMN "name" VARCHAR NOT NULL DEFAULT '';
