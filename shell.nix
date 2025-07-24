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

  buildInputs = with pkgs; let
    common = [
      nixd
      statix
      deadnix
      alejandra

      rustc
      cargo
      rustfmt
      clippy
      rust-analyzer
      cargo-watch

      pandoc
      diesel-cli

      openssl

      gtk4
      meson
      ninja
      pango
      gettext
      vte-gtk4
      pkg-config
      gdk-pixbuf
      libadwaita
      pkg-config
      libgweather
      gnome-desktop
      appstream-glib
      wrapGAppsHook4
      desktop-file-utils
      gobject-introspection
      rustPlatform.bindgenHook
    ];
    linux = [
      parted
      polkit
    ];
    darwin = [];
  in
    common
    ++ (
      if stdenv.isLinux
      then linux
      else if stdenv.isDarwin
      then darwin
      else []
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
