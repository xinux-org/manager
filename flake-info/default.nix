{
  pkgs ? import <nixpkgs> { },
}:
pkgs.rustPlatform.buildRustPackage rec {
  name = "flake-info";
  src = ./.;
  cargoLock = {
    lockFile = ./Cargo.lock;
  };
  nativeBuildInputs = with pkgs; [ pkg-config ];
  buildInputs =
    with pkgs;
    [
      openssl
      openssl.dev
      makeWrapper
    ]
    ++ lib.optional pkgs.stdenv.isDarwin [
      libiconv
      darwin.apple_sdk.frameworks.Security
    ];

  checkInputs = with pkgs; [ pandoc ];

  ROOTDIR = builtins.placeholder "out";
  LINK_MANPAGES_PANDOC_FILTER = import src/data/link-manpages.nix { inherit pkgs; };

  postInstall = ''
    cp -rt "$out" assets

    wrapProgram $out/bin/flake-info \
      --prefix PATH : ${pkgs.pandoc}/bin
  '';
}
