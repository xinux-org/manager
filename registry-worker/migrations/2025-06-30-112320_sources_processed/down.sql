-- This file should undo anything in `up.sql`
ALTER TABLE "git_host_sources" DROP COLUMN "processed";

ALTER TABLE "git_sources" DROP COLUMN "processed";



ALTER TABLE "nixpkgs_sources" DROP COLUMN "processed";






