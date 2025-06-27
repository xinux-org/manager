-- This file should undo anything in `up.sql`








ALTER TABLE "packages" DROP CONSTRAINT IF EXISTS "packages_unique_name" RESTRICT;
ALTER TABLE "packages" DROP COLUMN "name";
ALTER TABLE "packages" DROP COLUMN "description";
ALTER TABLE "packages" DROP COLUMN "homepage";
ALTER TABLE "packages" ADD COLUMN "description" VARCHAR NOT NULL;
ALTER TABLE "packages" ADD COLUMN "homepage" VARCHAR NOT NULL;

