{ inputs, pkgs, ... }:

{
  imports = [
    inputs.rust-flake.flakeModules.default
    inputs.rust-flake.flakeModules.nixpkgs
    inputs.process-compose-flake.flakeModule
    inputs.cargo-doc-live.flakeModule
  ];

  perSystem = { config, self', pkgs, lib, ... }: {
    rust-project = {
      crates."rich_pokedex".crane.args = {
        nativeBuildInputs = with pkgs; [
          makeWrapper
          pkg-config
        ];

        buildInputs = with pkgs; [ ]
          ++ lib.optionals pkgs.stdenv.isLinux
          [ alsa-lib.dev udev.dev alsa-utils ]
          ++ lib.optionals pkgs.stdenv.isDarwin
          (with pkgs.darwin.apple_sdk.frameworks; [ IOKit ]);
      };

      src = lib.cleanSourceWith {
        src = inputs.self; # The original, unfiltered source
        filter = path: type:
          (lib.hasInfix "/data/" path) ||
          # Default filter from crane (allow .rs files)
          (config.rust-project.crane-lib.filterCargoSources path type);
      };
    };

    packages.default = self'.packages.rich_pokedex.overrideAttrs (oldAttrs: {
      installPhase =
        (oldAttrs.installPhase or "") + ''
          mkdir -p $out/share/rich_pokedex/
          cp -r ./data/assets/ $out/share/rich_pokedex/
          cp ./data/pokedex.sqlite $out/share/rich_pokedex/
        '';

      postFixup =
        # `cd` to the directory containing assets (which is `bin/`, per the 
        # installPhase above) before launching the app.
        (oldAttrs.postFixup or "") + ''
          wrapProgram $out/bin/${oldAttrs.pname} \
            --chdir $out/bin
        '';
    });
  };
}
