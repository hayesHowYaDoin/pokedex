{ inputs, ... }:

{
  perSystem = { config, self', pkgs, lib, ... }:
    let
      rpkgs = import inputs.nixpkgs {
        inherit (pkgs) system;
        overlays = [ inputs.rust-overlay.overlays.default ];
      };
    in
    {
      devShells.default = rpkgs.mkShell rec {
        inputsFrom = [
          config.pre-commit.devShell # See ./nix/modules/pre-commit.nix
        ];

        nativeBuildInputs = with rpkgs; [
          pkg-config
        ];

        buildInputs = with rpkgs; [
          alsa-lib.dev
          bacon
          cargo-deb
          clippy
          just
          nixd
          sqlite
          udev.dev
          unzip
          wget
          rust-bin.stable.latest.default
        ];

        LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;

        shellHook = ''
          export POKEDEX_DATABASE_PATH="''$(${lib.getExe config.flake-root.package})/data/pokedex.sqlite"
          export POKEDEX_ASSETS_PATH="''$(${lib.getExe config.flake-root.package})/data/assets"
        '';
      };
    };
}
