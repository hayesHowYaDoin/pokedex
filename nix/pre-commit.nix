{ inputs, ... }:

{
  imports = [
    (inputs.git-hooks + /flake-module.nix)
  ];

  perSystem = { config, self', pkgs, lib, ... }: {
    pre-commit.settings = {
      hooks = {
        nixpkgs-fmt.enable = true;
        rustfmt.enable = true;
        markdownlint.enable = true;
        bats.enable = true;
        pretty-format-json.enable = true;
      };
    };
  };
}
