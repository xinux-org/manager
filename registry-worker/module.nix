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
  databaseName = "xinux-registry";
  localDatabase = ((cfg.database.host == "127.0.0.1") || (cfg.database.host == "localhost"));

  config1 =
    with lib;
    mkIf cfg.enable {
      users = {
        users.${cfg.user} = {
          isSystemUser = true;
          group = cfg.group;
        };
        groups.${cfg.group} = { };
      };

      services.postgresql = {
        enable = true;
        ensureUsers = [
          {
            name = databaseName;
            ensureDBOwnership = true;
          }
        ];
        ensureDatabases = [ databaseName ];
      };

      systemd.services.xinux-manager-registry-worker = {
        description = ''
          xinux manager registry-worker is responsible for fetching and storing data about packages, options and flakes from various places, especially from nixos/nixpkgs
        '';
        documentation = [ "https://github.com/xinux-org/manager" ];

        after = [ "network-online.target" ] ++ lib.optional localDatabase "postgresql.service";
        wants = [ "network-online.target" ];
        wantedBy = [ "multi-user.target" ];

        path = with pkgs; [
          coreutils
          replace-secret
          diesel-cli
          diesel-cli-ext
        ];

        serviceConfig = {
          User = cfg.user;
          Group = cfg.group;
          Restart = "on-failure";
          WorkingDirectory = "${cfg.dataDir}";
          ExecStartPre = ''
            ${lib.optionalString cfg.database.socketAuth ''
              ${pkgs.coreutils}/bin/echo "DATABASE_URL=postgres://${cfg.database.user}@/${cfg.database.name}?host=${cfg.database.socket}" > "${cfg.dataDir}/.env"
            ''}

            ${lib.optionalString (!cfg.database.socketAuth) ''
              ${pkgs.coreutils}/bin/echo "DATABASE_URL=postgres://${cfg.database.user}:#password#@${cfg.database.host}/${cfg.database.name}" > "${cfg.dataDir}/.env"
              ${pkgs.replace-secret}/bin/replace-secret '#password#' '${cfg.database.passwordFile}' '${cfg.dataDir}/.env'
            ''}

            ${pkgs.diesel-cli}/bin/diesel migration run
          '';
          ExecStart = "${lib.getBin cfg.package}/bin/registry-worker";
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
        default = databaseName;
      };

      group = mkOption {
        type = types.str;
        default = databaseName;
      };

      package = mkOption {
        type = types.package;
        default = pkg;
      };

      dataDir = mkOption {
        type = types.str;
        default = "/var/lib/xinux-manager/regstiry-worker";
      };

      database = {
        host = mkOption {
          type = types.str;
          default = "127.0.0.1";
          description = "Database host address. Leave \"127.0.0.1\" if you want local database";
        };

        socketAuth = mkOption {
          type = types.bool;
          default = if localDatabase then true else false;
          description = "Use Unix socket authentication for PostgreSQL instead of password authentication when local database wanted.";
        };

        socket = mkOption {
          type = types.nullOr types.path;
          default = if localDatabase then "/run/postgresql" else null;
          description = "Path to the PostgreSQL Unix socket.";
        };

        port = mkOption {
          type = types.port;
          default = config.services.postgresql.settings.port;
          defaultText = "5432";
          description = "Database host port.";
        };

        name = mkOption {
          type = types.str;
          default = "xinux-registry";
          description = "Database name.";
        };

        user = mkOption {
          type = types.str;
          default = "xinux-registry";
          description = "Database user.";
        };

        passwordFile = mkOption {
          type = types.nullOr types.path;
          default = null;
          example = "/run/keys/${manifest-name}-dbpassword";
          description = ''
            A file containing the password corresponding to
            {option}`database.user`.
          '';
        };
      };
    };
  };

  config = lib.mkMerge [ config1 ];
}
