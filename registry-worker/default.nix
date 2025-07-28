pkgs: let
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
  flake-info = ../flake-info;
  getLibFolder = pkg: "${pkg}/lib";
in
  pkgs.rustPlatform.buildRustPackage {
    pname = manifest.name;
    version = manifest.version;
    src = pkgs.lib.cleanSource ./.;
    cargoLock = {
      lockFile = ./Cargo.lock;
    };

    postPatch = ''
      sed -i "s|../flake-info|${flake-info}|g" Cargo.toml
    '';

    LINK_MANPAGES_PANDOC_FILTER = import ../flake-info/src/data/link-manpages.nix {inherit pkgs;};

    nativeBuildInputs = (
      with pkgs; [
        pkg-config
        openssl
        libgit2
      ]
    );

    buildInputs = (
      with pkgs; [
        pandoc
        sqlite.dev
        openssl
        libpq
      ]
    );

    NIX_LDFLAGS = "-L${(getLibFolder pkgs.sqlite.dev)} -L${(getLibFolder pkgs.libpq)}";

    fixupPhase = ''
      mkdir -p $out/lib
      cp -R ./migrations $out/lib
    '';
  }
