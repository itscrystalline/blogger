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
}
