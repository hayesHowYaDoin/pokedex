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
          just
          nixd
          sqlite
          udev.dev
          unzip
          wget
          (rust-bin.stable."1.88.0".default.override {
            extensions = [ "rust-src" "rust-analyzer" ];
            targets = [
              "aarch64-unknown-linux-gnu"
              "x86_64-unknown-linux-gnu"
            ];
          })
        ];

        LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;

        shellHook = ''
          export PROJECT_ROOT="''$(${lib.getExe config.flake-root.package})"
          export POKEDEX_DATABASE_PATH="''$(${lib.getExe config.flake-root.package})/data/pokedex.db"
          export POKEDEX_ASSETS_PATH="''$(${lib.getExe config.flake-root.package})/data/assets"
          export POKEDEX_LOG_PATH="''$(${lib.getExe config.flake-root.package})/logs/application.log"
          
          export DATABASE_URL="sqlite://$POKEDEX_DATABASE_PATH"

          cargo install cargo-deb
        '';
      };
    };
}
