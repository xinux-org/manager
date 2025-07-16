{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    registry-worker = {
      url = "path:registry-worker";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      registry-worker,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
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
