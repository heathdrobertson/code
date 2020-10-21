# see https://nixos.org/nixos/packages.html to search for more
with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "nix-repl";

  buildInputs = [];
  shellHook = ''
    nix repl
  '';
}
