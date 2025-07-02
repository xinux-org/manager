-- Your SQL goes here
ALTER TABLE "git_host_sources" ADD COLUMN "processed" BOOL NOT NULL DEFAULT false;

ALTER TABLE "git_sources" ADD COLUMN "processed" BOOL NOT NULL DEFAULT false;



ALTER TABLE "nixpkgs_sources" ADD COLUMN "processed" BOOL NOT NULL DEFAULT false;






