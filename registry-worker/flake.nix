{
  description = "xinux registry worker";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
        };
        package = pkgs.callPackage ./default.nix pkgs;
      in {
        packages = {
          registry-worker = package;
          default = package;
        };
        defaultPackage = package;
      }
    );
}
