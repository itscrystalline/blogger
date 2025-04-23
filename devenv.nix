{
  pkgs,
  lib,
  config,
  inputs,
  ...
}: {
  packages = [pkgs.cargo-tarpaulin];

  languages.rust = {
    enable = true;
    channel = "stable";
    components = ["rustc" "cargo" "clippy" "rustfmt" "rust-analyzer"];
  };

  scripts.test-codecov.exec = ''
    cargo tarpaulin --color always --skip-clean
  '';

  services.mysql = {
    enable = true;
    importTimeZones = true;
    ensureUsers = [
      {
        name = "blogger";
        password = "blogger";
        ensurePermissions = {
          "blogger.*" = "ALL PRIVILEGES";
        };
      }
    ];
    initialDatabases = [
      {
        name = "blogger";
        schema = ./schema.sql;
      }
    ];
  };
}
