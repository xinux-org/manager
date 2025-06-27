-- This file should undo anything in `up.sql`

ALTER TABLE "package_versions" DROP CONSTRAINT IF EXISTS "package_versions_unique_package_version" RESTRICT;
