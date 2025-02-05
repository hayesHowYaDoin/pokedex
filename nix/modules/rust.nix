{ inputs, ... }:

{
  imports = [
    inputs.rust-flake.flakeModules.default
    inputs.rust-flake.flakeModules.nixpkgs
    inputs.process-compose-flake.flakeModule
    inputs.cargo-doc-live.flakeModule
  ];

  perSystem = { config, self', pkgs, lib, ... }: {
    rust-project.crates."rich_pokedex".crane.args = {
      nativeBuildInputs = with pkgs; [ pkg-config ];
      # buildInputs = with pkgs; [ alsa-lib.dev udev.dev ];
      buildInputs = with pkgs; [
        alsa-lib.dev
        udev.dev
        alsa-utils
      ] ++ lib.optionals pkgs.stdenv.isDarwin (
        with pkgs.darwin.apple_sdk.frameworks; [
          IOKit
        ]
      );
    };
    packages.default = self'.packages.rich_pokedex;
  };
}
