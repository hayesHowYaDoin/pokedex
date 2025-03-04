{ inputs, ... }:

{
  perSystem = { config, self', pkgs, lib, ... }: {
    devShells.default = pkgs.mkShell rec {
      inputsFrom = [
        config.pre-commit.devShell # See ./nix/modules/pre-commit.nix
      ];

      nativeBuildInputs = with pkgs; [
        pkg-config
      ];

      buildInputs = with pkgs; [
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

      LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;

      shellHook = ''
        export POKEDEX_DATABASE_PATH="''$(${lib.getExe config.flake-root.package})/data/pokedex.sqlite"
        export POKEDEX_ASSETS_PATH="''$(${lib.getExe config.flake-root.package})/data/assets"

        rustup target add aarch64-unknown-linux-gnu
        rustup target add x86_64-unknown-linux-gnu
      '';
    };
  };
}
