-- Your SQL goes here

ALTER TABLE "nixpkgs_sources" ADD CONSTRAINT "nixpkgs_sources_unique_sha" UNIQUE ("sha");
