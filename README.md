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
- [ ] Ability to process whole nixpkgs
- [ ] Ability to monitor and process latest nixpkgs channels/revisions
- [ ] Ability to peek historical versions/channels

.
