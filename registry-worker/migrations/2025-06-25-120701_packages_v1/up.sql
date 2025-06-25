CREATE TABLE "maintainers"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"name" VARCHAR NOT NULL,
	"github" VARCHAR,
	"email" VARCHAR
);

CREATE TABLE "revisions"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"source_id" INT4 NOT NULL
);

CREATE TABLE "licenses"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"name" VARCHAR NOT NULL,
	"license" VARCHAR,
	"fullname" VARCHAR,
	"shortname" VARCHAR,
	"url" VARCHAR
);

CREATE TABLE "sources"(
	"id" INT4 NOT NULL PRIMARY KEY
);

CREATE TABLE "packages"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"description" VARCHAR NOT NULL,
	"homepage" VARCHAR NOT NULL
);

CREATE TABLE "platforms"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"name" VARCHAR NOT NULL
);

CREATE TABLE "package_versions"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"revision_id" INT4 NOT NULL,
	"package_id" INT4 NOT NULL,
	"license_id" INT4 NOT NULL,
	"available" BOOL NOT NULL,
	"broken" BOOL NOT NULL,
	"insecure" BOOL NOT NULL,
	"changelog" VARCHAR NOT NULL,
	"version" VARCHAR NOT NULL,
	FOREIGN KEY ("revision_id") REFERENCES "revisions"("id"),
	FOREIGN KEY ("package_id") REFERENCES "packages"("id"),
	FOREIGN KEY ("license_id") REFERENCES "licenses"("id")
);

CREATE TABLE "package_versions_platforms"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"package_version_id" INT4 NOT NULL,
	"platform_id" INT4 NOT NULL,
	FOREIGN KEY ("package_version_id") REFERENCES "package_versions"("id"),
	FOREIGN KEY ("platform_id") REFERENCES "platforms"("id")
);

CREATE TABLE "package_versions_maintainers"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"package_version_id" INT4 NOT NULL,
	"maintainer_id" INT4 NOT NULL,
	FOREIGN KEY ("package_version_id") REFERENCES "package_versions"("id"),
	FOREIGN KEY ("maintainer_id") REFERENCES "maintainers"("id")
);
