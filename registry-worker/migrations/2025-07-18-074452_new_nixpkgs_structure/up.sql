-- Your SQL goes here
ALTER TABLE "git_host_sources" ADD COLUMN "created_at" TIMESTAMP NOT NULL DEFAULT NOW();
ALTER TABLE "git_host_sources" ADD COLUMN "locked_at" TIMESTAMP;

ALTER TABLE "git_sources" ADD COLUMN "created_at" TIMESTAMP NOT NULL DEFAULT NOW();
ALTER TABLE "git_sources" ADD COLUMN "locked_at" TIMESTAMP;

ALTER TABLE "licenses" ADD COLUMN "created_at" TIMESTAMP NOT NULL DEFAULT NOW();

ALTER TABLE "maintainers" ADD COLUMN "created_at" TIMESTAMP NOT NULL DEFAULT NOW();

ALTER TABLE "nixpkgs_sources" DROP COLUMN "channel";
ALTER TABLE "nixpkgs_sources" DROP COLUMN "git_ref";
ALTER TABLE "nixpkgs_sources" ADD COLUMN "committed_at" TIMESTAMP NOT NULL DEFAULT NOW();
ALTER TABLE "nixpkgs_sources" ADD COLUMN "created_at" TIMESTAMP NOT NULL DEFAULT NOW();
ALTER TABLE "nixpkgs_sources" ADD COLUMN "locked_at" TIMESTAMP;
ALTER TABLE "nixpkgs_sources" ADD COLUMN "sha" VARCHAR NOT NULL DEFAULT '';

ALTER TABLE "package_versions" ADD COLUMN "created_at" TIMESTAMP NOT NULL DEFAULT NOW();



ALTER TABLE "packages" ADD COLUMN "created_at" TIMESTAMP NOT NULL DEFAULT NOW();

ALTER TABLE "platforms" ADD COLUMN "created_at" TIMESTAMP NOT NULL DEFAULT NOW();

CREATE TABLE "nixpkgs_channels"(
	"id" SERIAL PRIMARY KEY,
	"name" VARCHAR NOT NULL,
	"created_at" TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE "nixpkgs_channels_sources"(
	"id" SERIAL PRIMARY KEY,
	"channel_id" INT4 NOT NULL,
	"source_id" INT4 NOT NULL,
	FOREIGN KEY ("channel_id") REFERENCES "nixpkgs_channels"("id"),
	FOREIGN KEY ("source_id") REFERENCES "nixpkgs_sources"("id"),
	CONSTRAINT "nixpkgs_channels_sources_unique" UNIQUE ("channel_id", "source_id")
);

