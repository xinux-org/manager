CREATE TABLE "nixpkgs_source_refs" (
  "id" SERIAL PRIMARY KEY,
  "git_ref" varchar,
  "commit_date" datetime,
  "fetch_date" datetime
);

CREATE TABLE "platforms" (
  "id" SERIAL PRIMARY KEY,
  "name" varchar
);

CREATE TABLE "outputs" (
  "id" SERIAL PRIMARY KEY,
  "name" varchar
);

CREATE TABLE "licenses" (
  "id" SERIAL PRIMARY KEY,
  "full_name" varchar,
  "short_name" varchar,
  "url" varchar
);

CREATE TABLE "attribute_sets" (
  "id" SERIAL PRIMARY KEY,
  "name" varchar
);

CREATE TABLE "programs" (
  "id" SERIAL PRIMARY KEY,
  "name" varchar
);

  CREATE TABLE "maintainers" (
    "id" SERIAL PRIMARY KEY,
    "name" varchar,
    "github" varchar,
    "email" varchar
  );

  CREATE TABLE "packages" (
    "id" SERIAL PRIMARY KEY,
    "attr_name" varchar,
    "pname" varchar,
    "pversion" varchar,
    "attribute_set_id" INTEGER,
    "nixpkgs_source_id" INTEGER,
    "default_output_id" INTEGER,
    "description" varchar,
    "long_description" varchar,
    "system_id" INTEGER
  );

  CREATE TABLE "packages_platforms" (
    "id" SERIAL PRIMARY KEY,
    "package_id" INTEGER,
    "platform_id" INTEGER
  );

  CREATE TABLE "packages_outputs" (
    "id" SERIAL PRIMARY KEY,
    "package_id" INTEGER,
    "output_id" INTEGER
  );

  CREATE TABLE "packages_programs" (
    "id" SERIAL PRIMARY KEY,
    "package_id" INTEGER,
    "program_id" INTEGER
  );

  CREATE TABLE "packages_licenses" (
    "id" SERIAL PRIMARY KEY,
    "package_id" INTEGER,
    "license_id" INTEGER
  );

  CREATE TABLE "packages_maintainers" (
    "id" SERIAL PRIMARY KEY,
    "package_id" INTEGER,
    "maintainer_id" INTEGER
  );

  CREATE TABLE "packages_homepages" (
    "id" SERIAL PRIMARY KEY,
    "package_id" INTEGER,
    "homepage" varchar
  );

  CREATE TABLE "packages_posititons" (
    "id" SERIAL PRIMARY KEY,
    "package_id" INTEGER,
    "position" varchar
  );

  CREATE TABLE "apps" (
    "id" SERIAL PRIMARY KEY,
    "attr_name" varchar,
    "type" varchar,
    "bin" varchar
  );

  CREATE TABLE "apps_platforms" (
    "id" SERIAL PRIMARY KEY,
    "app_id" INTEGER,
    "platform_id" INTEGER
);

CREATE TABLE "options" (
  "id" SERIAL PRIMARY KEY,
  "source" varchar,
  "name" varchar,
  "description" varchar,
  "type" varchar,
  "default_value" varchar,
  "example" varchar,
  "flake1" varchar,
  "flake2" varchar
);

ALTER TABLE "packages" ADD FOREIGN KEY ("attribute_set_id") REFERENCES "attribute_sets" ("id");

ALTER TABLE "packages" ADD FOREIGN KEY ("nixpkgs_source_id") REFERENCES "nixpkgs_source_refs" ("id");

ALTER TABLE "packages" ADD FOREIGN KEY ("default_output_id") REFERENCES "outputs" ("id");

ALTER TABLE "packages" ADD FOREIGN KEY ("system_id") REFERENCES "platforms" ("id");

ALTER TABLE "packages_platforms" ADD FOREIGN KEY ("package_id") REFERENCES "packages" ("id");

ALTER TABLE "packages_platforms" ADD FOREIGN KEY ("platform_id") REFERENCES "platforms" ("id");

ALTER TABLE "packages_outputs" ADD FOREIGN KEY ("package_id") REFERENCES "packages" ("id");

ALTER TABLE "packages_outputs" ADD FOREIGN KEY ("output_id") REFERENCES "outputs" ("id");

ALTER TABLE "packages_programs" ADD FOREIGN KEY ("package_id") REFERENCES "packages" ("id");

ALTER TABLE "packages_programs" ADD FOREIGN KEY ("program_id") REFERENCES "programs" ("id");

ALTER TABLE "packages_licenses" ADD FOREIGN KEY ("package_id") REFERENCES "packages" ("id");

ALTER TABLE "packages_licenses" ADD FOREIGN KEY ("license_id") REFERENCES "licenses" ("id");

ALTER TABLE "packages_maintainers" ADD FOREIGN KEY ("package_id") REFERENCES "packages" ("id");

ALTER TABLE "packages_maintainers" ADD FOREIGN KEY ("maintainer_id") REFERENCES "maintainers" ("id");

ALTER TABLE "packages_homepages" ADD FOREIGN KEY ("package_id") REFERENCES "packages" ("id");

ALTER TABLE "packages_posititons" ADD FOREIGN KEY ("package_id") REFERENCES "packages" ("id");

ALTER TABLE "apps_platforms" ADD FOREIGN KEY ("app_id") REFERENCES "apps" ("id");

ALTER TABLE "apps_platforms" ADD FOREIGN KEY ("platform_id") REFERENCES "platforms" ("id");

