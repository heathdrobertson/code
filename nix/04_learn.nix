# This import the nix package collection,
# so we can accwss the `pkgs` and `stdenv` variables.
with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "my-environment";

  # The packages in the `buildInputs` list iwll be added to the PATH in our shell.
  buildInputs = [
    # cowsay is an arbitrary package
    # see http://nixos.org/nixos/packages.html to search for more.
    pkgs.cowsay
    pkgs.lolcat
    pkgs.figlet
  ];

  # The '' quotes are 2 single quote characters
  # Thery are sued for mulit-line strings.
  # The shellHook property is very useful for setting environment variables and the like.
  shellHook = ''
    figlet "ToiletHill!" | lolcat --freq 0.9
    echo "Welcome to the nix environment!" | cowsay | lolcat --freq 0.9

    '';
}
