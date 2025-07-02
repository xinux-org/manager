pkgs:
let
  rust_channel = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
in
pkgs.mkShell {
  nativeBuildInputs = (
    with pkgs;
    [
      pkg-config
      openssl
      libgit2
      sqlite
      protobuf
      libpq
    ]
  );

  buildInputs = (
    with pkgs;
    [
      flutter
      android-tools
      pandoc
      rust-bin.beta.latest.default
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
