# Xinux Manager - manage your Nix configuration easily

DISCLAIMER: this project is in early stage. don't expect anything working unless stable release

WANT TO DO:

- [ ] search for packages indexed from nixpkgs (this means we need own server with indexing directly from nixpkgs)
- [ ] generate config for nixos
- [ ] manage system user
- [ ] home-manager settings
- [ ] store config in github

## registry-worker

### What should be done here

The main idea is to store all available packages and options from nixpkgs
just like search.nixos.org does. But unlike search.nixos.org we should also
store older revisions of all packages. This gives us ability to search
specific versions of specific packages and knowledge what exactly versions
are coming after the current (very likely old) version of a program.
mynixos.com is very similar to search.nixos.org. And nixhub.io is the
closest match I found.

In order to accomplish our goals we need to work on following paths:

1. Functionality to fetch all of nixpkgs information out of github url and revision
   (thanks to search.nixos.org this functionality is almost finished)

2. Functionality to check github every n seconds/minutes for the newest revision
   and queue it for fetching

3. Functionality to fetch historical revisions and work on them with k workers in parallel

4. After any kind of revision fetch, we should store them in a PostgreSQL and Elastic Search
   for further processing


### TODO:

- [x] Make database scheme (try to normalize it)
- [x] Use diesel sql
- [x] Ability to process test firefox asset and store it in db
- [ ] Process all available fields of a package and package version
- [ ] Ability to process multiple packages with different versions and/or duplicate ones
- [x] Ability to process whole nixpkgs
- [ ] Ability to monitor and process latest nixpkgs channels/revisions
- [ ] Ability to peek historical versions/channels

## diesel guide:

### If you updated schema.rs

- `diesel migration generate --diff-schema <migration_name>`

### If you want to create migration from scratch

- `diesel migration generate <migration_name>`
- do reviews

### Reviewing freshly created migration(s)

- review `up.sql` and `down.sql`
   - if you create a new table or delete existing, make shure that `"id" SERIAL PRIMARY KEY` is proprely setted up
   - make shure that you setted up `FOREIGN KEY` and `UNIQUE` constraints properly
- try to fill a database with some data
- make shure that all tables have some data
- try to run `diesel migration run` in order to apply `up.sql`
- try to run `diesel migration revert` in order to apply `down.sql`

### When pulled latest commits from git

- run `diesel migration run`
