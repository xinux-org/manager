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
            name = cfg.database.user;
            ensureDBOwnership = true;
          }
        ];
        ensureDatabases = [ cfg.database.name ];

        identMap = ''
          # mapping name       system user database user
          ${cfg.database.user} ${cfg.user} ${cfg.database.user}
        '';

        authentication = pkgs.lib.mkAfter ''
          # scope database name        database user        method mapping name
            local ${cfg.database.name} ${cfg.database.user} peer   map=${cfg.database.user}
        '';
      };

      systemd.services.xinux-manager-registry-worker = {
        description = ''
          xinux manager registry-worker is responsible for fetching and storing data about packages, options and flakes from various places, especially from nixos/nixpkgs
        '';
        documentation = [ "https://github.com/xinux-org/manager" ];

        after = [ "network-online.target" ] ++ lib.optional localDatabase "postgresql.service";
        requires = lib.optional localDatabase "postgresql.service";
        wants = [ "network-online.target" ];
        wantedBy = [ "multi-user.target" ];

        path =
          with pkgs;
          [
            coreutils
            replace-secret
            diesel-cli
            diesel-cli-ext
          ]
          ++ [ cfg.package ];

        serviceConfig = {
          User = cfg.user;
          Group = cfg.group;
          Restart = "on-failure";
          ExecStartPre = pkgs.writeScript "xinux-manager-registry-worker-pre-start.sh" ''
            #!${pkgs.runtimeShell}

            ${lib.optionalString cfg.database.socketAuth ''
              echo "DATABASE_URL=postgres://${cfg.database.user}/${cfg.database.name}?host=${cfg.database.socket}" > /var/lib/${cfg.dataDir}/.env
            ''}
            ${lib.optionalString (!cfg.database.socketAuth) ''
              echo "DATABASE_URL=postgres://${cfg.database.user}:#password#@${cfg.database.host}/${cfg.database.name}" > /var/lib/${cfg.dataDir}/.env
              replace-secret "#password#" ${cfg.database.passwordFile} /var/lib/${cfg.dataDir}/.env
            ''}

            source /var/lib/${cfg.dataDir}/.env
            diesel migration run --migration-dir "${pkg}/lib/migrations"
          '';
          StateDirectory = "${cfg.dataDir}";
          StateDirectoryMode = "0750";
          WorkingDirectory = "/var/lib/${cfg.dataDir}";
          ExecStart = "${cfg.package}/bin/registry-worker";
          ExecReload = "${pkgs.coreutils}/bin/kill -s HUP $MAINPID";
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
        default = "xinux-manager";
      };

      group = mkOption {
        type = types.str;
        default = "xinux-manager";
      };

      package = mkOption {
        type = types.package;
        default = pkg;
      };

      dataDir = mkOption {
        type = types.str;
        default = "xinux-manager/registry-worker";
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
          example = "/run/keys/your-dbpassword";
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
