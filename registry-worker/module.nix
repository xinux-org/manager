flake:
{
  config,
  lib,
  pkgs,
  ...
}:
let
  cfg = config.services.xinux-manager.registry-worker;
  system = pkgs.stdenv.hostPlatform.system;
  pkg = flake.packages.${system}.default;

  config1 =
    with lib;
    mkIf cfg.enable {
      users = {
        users.${cfg.user} = {
          description = "Xinux Manager User";
          isSystemUser = true;
          group = cfg.group;
        };
        groups.${cfg.group} = { };
      };

      systemd.services.xinux-manager-regstiry-worker = {
        description = ''
          xinux manager registry-worker is responsible for fetching and storing data about packages, options and flakes from various places, especially from nixos/nixpkgs
        '';
        documentation = [ "https://github.com/xinux-org/manager" ];

        after = [ "network-online.target" ];
        wants = [ "network-online.target" ];
        wantedBy = [ "multi-user.target" ];

        serviceConfig = {
          User = cfg.user;
          Group = cfg.group;
          Restart = "always";
          ExecStart = "${lib.getBin cfg.package}/bin/registry-worker ${genArgs { cfg = cfg; }}";
          StateDirectory = cfg.user;
          StateDirectoryMode = "0750";
        };
      };
    };
in
{
  options = with lib; {
    services.xinux-manager.registry-worker = {
      enable = mkEnableOption ''
        xinux manager registry-worker is responsible for fetching and storing data about packages, options and flakes from various places, especially from nixos/nixpkgs
      '';

      user = mkOption {
        type = types.str;
        default = "xinux-manager-regstiry-worker";
      };

      group = mkOption {
        type = types.str;
        default = "xinux-manager";
      };

      package = mkOption {
        type = types.package;
        default = pkg;
      };

      db = {
        host = mkOption {
          type = types.str;
          default = "localhost";
        };
        port = mkOption {
          type = types.number;
          default = 5432;
        };
        database = mkOption {
          type = types.str;
          default = "registry";
        };
        username = mkOption {
          type = types.str;
          default = "username";
        };
        password = mkOption {
          type = types.str;
          default = "password";
        };
      };
    };
  };

  config = lib.mkMerge [ config1 ];
}
