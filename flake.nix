{
  description = ''
    A Pokedex CLI application which takes full advantage of modern terminals.
  '';

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    flake-parts.inputs.nixpkgs-lib.follows = "nixpkgs";
    systems.url = "github:nix-systems/default";
    rust-flake.url = "github:juspay/rust-flake";
    rust-flake.inputs.nixpkgs.follows = "nixpkgs";
    process-compose-flake.url = "github:Platonic-Systems/process-compose-flake";
    cargo-doc-live.url = "github:srid/cargo-doc-live";
    flake-root.url = "github:srid/flake-root";

    git-hooks.url = "github:cachix/git-hooks.nix";
    git-hooks.flake = false;
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;

      # See ./nix/modules/*.nix for the modules that are imported here.
      imports = [
        inputs.flake-root.flakeModule
      ] ++ (with builtins;
        map
          (fn: ./nix/${fn})
          (attrNames (readDir ./nix)));
    };
}
