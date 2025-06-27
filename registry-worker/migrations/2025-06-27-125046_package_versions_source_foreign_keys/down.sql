-- This file should undo anything in `up.sql`

ALTER TABLE "package_versions" DROP CONSTRAINT IF EXISTS "package_versions_nixpkgs_source_id_fkey";
ALTER TABLE "package_versions" DROP CONSTRAINT IF EXISTS "package_versions_git_host_source_id_fkey";
ALTER TABLE "package_versions" DROP CONSTRAINT IF EXISTS "package_versions_git_source_id_fkey";
