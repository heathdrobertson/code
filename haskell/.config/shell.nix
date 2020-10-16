{ nixpkgs ? import <nixpkgs> {} }:
let
  inherit (nixpkgs) pkgs;
  inherit (pkgs) haskellPackages;

  haskellDeps = ps: with ps; [
    base
    lens
    mtl
  ];

  ghc = haskellPackages.ghcWithPackages haskellDeps;

  nixPackages = [
    pkgs.wget
    ghc
    pkgs.gdb
    haskellPackages.cabal-install
    pkgs.cabal2nix
  ];
in
pkgs.stdenv.mkDerivation {
  name = "env";
  buildInputs = nixPackages;
}
