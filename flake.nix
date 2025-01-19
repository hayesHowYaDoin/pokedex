{
  description = ''
    A Pokedex CLI application which takes full advantage of modern terminals.
  '';

  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";

  outputs = { self, nixpkgs }:
    let
      inherit (nixpkgs) lib;

      forAllSystems = lib.genAttrs lib.systems.flakeExposed;
    in {
      devShells = forAllSystems (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in {
          default = pkgs.mkShell {
            buildInputs = with pkgs; [
              alsa-lib
              alsa-utils
              cargo
              clippy
              gcc
              rustfmt
              rustc
              rust-analyzer
              sqlite
              wget
            ];
            
            nativeBuildInputs = with pkgs; [ pkg-config ];
          };
        });
    };
}