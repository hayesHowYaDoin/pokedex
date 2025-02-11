{ inputs, ... }:

{
  perSystem = { config, self', pkgs, lib, ... }: {
    devShells.default = pkgs.mkShell {
      name = "rich_pokedex_shell";

      inputsFrom = [
        # self'.devShells.rust
        config.pre-commit.devShell # See ./nix/modules/pre-commit.nix
      ];

      packages = with pkgs; [
        cargo-deb
        cargo-cross
        rustup
        just
        nixd
        bacon
        alsa-lib
        alsa-utils
        sqlite
        wget
        unzip
      ];

      shellHook = ''
        export POKEDEX_DATABASE_PATH="''$(${lib.getExe config.flake-root.package})/data/pokedex.sqlite"
        export POKEDEX_ASSETS_PATH="''$(${lib.getExe config.flake-root.package})/data/assets"

        rustup target add aarch64-unknown-linux-gnu
        rustup target add x86_64-unknown-linux-gnu
      '';
    };
  };
}
