let pinned-nixpkgs-path = import ./pinned-nixpkgs.nix;
    pinned-pkgs = import pinned-nixpkgs-path {};
in { pkgs ? pinned-pkgs }:

with pkgs;

mkShell {
  buildInputs = [ cargo rustc rustfmt ];
  PINNED_NIX_PATH = pinned-nixpkgs-path;
  NIX_PATH="nixpkgs=${pinned-nixpkgs-path}";
}
