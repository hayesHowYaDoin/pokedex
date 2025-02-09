{ inputs, ... }:

{
  perSystem = { config, self', pkgs, lib, ... }: {
    devShells.default = pkgs.mkShell {
      name = "rich_pokedex_shell";

      inputsFrom = [
        self'.devShells.rust
        config.pre-commit.devShell # See ./nix/modules/pre-commit.nix
      ];

      packages = with pkgs; [
        cargo-deb
        just
        nixd
        bacon
        alsa-lib
        alsa-utils
        sqlite
        wget
      ];

      shellHook = ''
        export POKEDEX_DATABASE_PATH="''$(${lib.getExe config.flake-root.package})/data/pokedex.sqlite"
        export POKEDEX_ASSETS_PATH="''$(${lib.getExe config.flake-root.package})/data/assets"
      '';
    };
  };
}
