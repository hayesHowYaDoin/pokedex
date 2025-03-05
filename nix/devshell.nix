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
        alsa-lib.dev
        bacon
        cargo
        cargo-deb
        clippy
        just
        nixd
        rustc
        rustfmt
        rustup
        sqlite
        udev.dev
        unzip
        wget
      ];

      LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;

      shellHook = ''
        export POKEDEX_DATABASE_PATH="''$(${lib.getExe config.flake-root.package})/data/pokedex.sqlite"
        export POKEDEX_ASSETS_PATH="''$(${lib.getExe config.flake-root.package})/data/assets"

        rustup target add aarch64-unknown-linux-gnu
        rustup target add x86_64-unknown-linux-gnu

        rustup component add rust-analyzer
        rust componenst add rust-src
      '';
    };
  };
}
