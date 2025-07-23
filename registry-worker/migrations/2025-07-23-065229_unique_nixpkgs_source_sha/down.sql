-- This file should undo anything in `up.sql`

ALTER TABLE "nixpkgs_sources" DROP CONSTRAINT IF EXISTS "nixpkgs_sources_unique_sha" RESTRICT;
