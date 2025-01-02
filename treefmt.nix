{ ... }:
{
  projectRootFile = "flake.nix";
  programs = {
    actionlint.enable = true;
    mdformat.enable = true;
    nixfmt.enable = true;
    rustfmt.enable = true;
    taplo.enable = true;
    yamlfmt.enable = true;
  };
}
