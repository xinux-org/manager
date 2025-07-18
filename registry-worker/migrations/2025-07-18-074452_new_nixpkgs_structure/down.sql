-- This file should undo anything in `up.sql`
ALTER TABLE "git_host_sources" DROP COLUMN "created_at";
ALTER TABLE "git_host_sources" DROP COLUMN "locked_at";

ALTER TABLE "git_sources" DROP COLUMN "created_at";
ALTER TABLE "git_sources" DROP COLUMN "locked_at";

ALTER TABLE "licenses" DROP COLUMN "created_at";

ALTER TABLE "maintainers" DROP COLUMN "created_at";

ALTER TABLE "nixpkgs_sources" DROP COLUMN "committed_at";
ALTER TABLE "nixpkgs_sources" DROP COLUMN "created_at";
ALTER TABLE "nixpkgs_sources" DROP COLUMN "locked_at";
ALTER TABLE "nixpkgs_sources" DROP COLUMN "sha";
ALTER TABLE "nixpkgs_sources" ADD COLUMN "channel" VARCHAR NOT NULL DEFAULT '';
ALTER TABLE "nixpkgs_sources" ADD COLUMN "git_ref" VARCHAR NOT NULL DEFAULT '';

ALTER TABLE "package_versions" DROP COLUMN "created_at";



ALTER TABLE "packages" DROP COLUMN "created_at";

ALTER TABLE "platforms" DROP COLUMN "created_at";

DROP TABLE IF EXISTS "nixpkgs_channels_sources";
DROP TABLE IF EXISTS "nixpkgs_channels";
