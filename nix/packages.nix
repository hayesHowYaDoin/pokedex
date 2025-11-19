{ inputs, ... }:

{
  perSystem = { config, self', pkgs, lib, system, ... }:
    let
      src = pkgs.lib.fileset.toSource {
        root = ./../.;
        fileset = pkgs.lib.fileset.unions [
          ./../src
          ./../lib
          ./../data
          ./../.sqlx
          ./../Cargo.toml
          ./../Cargo.lock
        ];
      };

      rich_pokedex = pkgs.rustPlatform.buildRustPackage {
        pname = "rich_pokedex";
        version = "1.1.0";
        inherit src;

        cargoLock = {
          lockFile = src + /Cargo.lock;
          outputHashes = {
            "crud-core-1.0.0" = "sha256-tSvRqGOF+5UUeHWi/Q33d2VPsi1GvX9dV23HAwN1+5M=";
            "crud-macro-1.0.0" = "sha256-tSvRqGOF+5UUeHWi/Q33d2VPsi1GvX9dV23HAwN1+5M=";
            "crud-macro-core-1.0.0" = "sha256-tSvRqGOF+5UUeHWi/Q33d2VPsi1GvX9dV23HAwN1+5M=";
          };
        };

        nativeBuildInputs = with pkgs; [
          pkg-config
        ];

        buildInputs = with pkgs; [
          alsa-lib.dev
          sqlite
          udev.dev
        ];

        LD_LIBRARY_PATH = lib.makeLibraryPath (with pkgs; [
          alsa-lib
          sqlite
          udev
        ]);

        SQLX_OFFLINE = "true";

        postInstall = ''
          # Create share directory for data
          mkdir -p $out/share/rich_pokedex

          # Generate database from SQL schema
          ${pkgs.sqlite}/bin/sqlite3 $out/share/rich_pokedex/pokedex.db < ${src}/data/pokedex.sql

          # Copy assets from source
          cp -r ${src}/data/assets $out/share/rich_pokedex/

          # Create logs directory
          mkdir -p $out/var/log
        '';
      };
    in
    {
      packages = {
        default = rich_pokedex;
        rich_pokedex = rich_pokedex;
      };

      apps = {
        default = {
          type = "app";
          program = lib.getExe rich_pokedex;
        };
      };
    };
}
