-- Your SQL goes here

ALTER TABLE "package_versions" ADD FOREIGN KEY ("nixpkgs_source_id") REFERENCES "nixpkgs_sources"("id");
ALTER TABLE "package_versions" ADD FOREIGN KEY ("git_host_source_id") REFERENCES "git_host_sources"("id");
ALTER TABLE "package_versions" ADD FOREIGN KEY ("git_source_id") REFERENCES "git_sources"("id");
