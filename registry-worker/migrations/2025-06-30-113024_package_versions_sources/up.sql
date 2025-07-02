-- Your SQL goes here





ALTER TABLE "package_versions" DROP COLUMN "nixpkgs_source_id";
ALTER TABLE "package_versions" DROP COLUMN "git_source_id";
ALTER TABLE "package_versions" DROP COLUMN "git_host_source_id";





CREATE TABLE "package_versions_sources"(
	"id" SERIAL PRIMARY KEY,
	"package_version_id" INT4 NOT NULL,
	"nixpkgs_source_id" INT4,
	"git_source_id" INT4,
	"git_host_source_id" INT4,
	FOREIGN KEY ("package_version_id") REFERENCES "package_versions"("id"),
	FOREIGN KEY ("nixpkgs_source_id") REFERENCES "nixpkgs_sources"("id"),
	FOREIGN KEY ("git_source_id") REFERENCES "git_sources"("id"),
	FOREIGN KEY ("git_host_source_id") REFERENCES "git_host_sources"("id")
);

