pkgs:
pkgs.mkShell {
  nativeBuildInputs = (
    with pkgs; [
      pkg-config
      openssl

      cmake
      llvmPackages.llvm
      llvmPackages.clang

      libgit2
      sqlite
      protobuf
      libpq
    ]
  );

  buildInputs = (
    with pkgs; [
      nixd
      statix
      deadnix
      alejandra

      pandoc
      diesel-cli

      rustc
      cargo
      rust-analyzer
      rustfmt
      clippy
      cargo-watch
    ]
  );

  RUST_BACKTRACE = "full";
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

  LD_LIBRARY_PATH = (
    with pkgs;
      lib.makeLibraryPath [
        openssl
        libgit2
        sqlite
        protobuf
      ]
  );

  LINK_MANPAGES_PANDOC_FILTER = import ./flake-info/src/data/link-manpages.nix {inherit pkgs;};

  LC_CTYPE = "en_US.UTF-8";
  LC_ALL = "en_US.UTF-8";
}
