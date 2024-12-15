let
  rust_overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  nixpkgs = import <nixpkgs> { overlays = [ rust_overlay ]; };
  rust_channel = nixpkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
  getLibFolder = pkg: "${pkg}/lib";
  libiconvPath = "${nixpkgs.libiconv}/lib";
in
with nixpkgs;

pkgs.mkShell {
  buildInputs = (with pkgs; [
    flutter
    android-tools
  ]);
}
