-- Your SQL goes here


ALTER TABLE "package_versions" DROP COLUMN "revision_id";
ALTER TABLE "package_versions" ADD COLUMN "source_id" INT4 NOT NULL;





DROP TABLE IF EXISTS "revisions";
ALTER TABLE "sources" ADD COLUMN "nixpkgs_source_id" INT4;
ALTER TABLE "sources" ADD COLUMN "git_host_source_id" INT4;
ALTER TABLE "sources" ADD COLUMN "git_source_id" INT4;

CREATE TABLE "git_sources"(
	"id" SERIAL PRIMARY KEY,
	"url" VARCHAR NOT NULL,
	CONSTRAINT "git_sources_unique" UNIQUE ("url")
);

CREATE TABLE "git_host_sources"(
	"id" SERIAL PRIMARY KEY,
	"host" VARCHAR NOT NULL,
	"owner" VARCHAR NOT NULL,
	"repo" VARCHAR NOT NULL,
	"git_ref" VARCHAR NOT NULL,
	CONSTRAINT "git_host_sources_unique" UNIQUE ("host", "owner", "repo", "git_ref")
);

CREATE TABLE "nixpkgs_sources"(
	"id" SERIAL PRIMARY KEY,
	"channel" VARCHAR NOT NULL,
	"git_ref" VARCHAR NOT NULL,
	CONSTRAINT "nixpkgs_sources_unique" UNIQUE ("channel", "git_ref")
);

