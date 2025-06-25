let
  rust_overlay = import (
    builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"
  );
  nixpkgs = import <nixpkgs> { overlays = [ rust_overlay ]; };
  rust_channel = nixpkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
in
with nixpkgs;
pkgs.mkShell {
  nativeBuildInputs =
    [ pkg-config ]
    ++ (with pkgs; [
      openssl
      libgit2
      sqlite
      protobuf
      libpq
    ]);

  buildInputs = (
    with pkgs;
    [
      flutter
      android-tools
      pandoc
      rustup
      diesel-cli
    ]
  );

  LD_LIBRARY_PATH = (
    with pkgs;
    lib.makeLibraryPath [
      openssl
      libgit2
      sqlite
      protobuf
    ]
  );

  RUST_SRC_PATH = "${rust_channel}/lib/rustlib/src/rust/library";
  LINK_MANPAGES_PANDOC_FILTER = import ./flake-info/src/data/link-manpages.nix { inherit pkgs; };
  FLUTTER_ROOT = "${pkgs.flutter}";

  LC_CTYPE = "en_US.UTF-8";
  LC_ALL = "en_US.UTF-8";

  shellHook = ''
    export PATH="$PATH:${rust_channel}/bin"
  '';
}
