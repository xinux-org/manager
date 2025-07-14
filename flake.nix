{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    registry-worker = {
      url = "path:registry-worker";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      registry-worker,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells.default = pkgs.callPackage ./shell.nix pkgs;
        packages = {
          registry-worker = registry-worker.defaultPackage.${system};
        };
      }
    )
    // {
      nixosModules = {
        registry-worker = import ./registry-worker/module.nix registry-worker;
      };
    };
}
