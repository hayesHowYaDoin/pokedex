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

          csvtosqlite = pkgs.rustPlatform.buildRustPackage rec {
            name = "csvtosqlite";
            tag = "main";

            # buildInputs = with pkgs; [ openssl ];

            nativeBuildInputs = with pkgs; [ pkg-config ];

            src = pkgs.fetchFromGitHub {
              owner = "mustakimali";
              repo = name;
              rev = "da8369c74119a2e6af9e54e9517d2974edad2490";
              sha256 = "sha256-nDzTOH/Ps1NQ7MNNz/v0ckYYIRlXQaqxt4ZPHFomGRk=";
            };

            cargoHash = "sha256-ef1rIbSRACeMi/h0b0Fdrz1QSiSIFWR/eXIkleNkCbo=";
          };
        in {
          default = pkgs.mkShell {
            buildInputs = with pkgs; [  alsa-lib alsa-utils cargo clippy csvtosqlite gcc rustfmt rustc rust-analyzer sqlite wget ];
            nativeBuildInputs = with pkgs; [ pkg-config ];
          };
        });
    };
}