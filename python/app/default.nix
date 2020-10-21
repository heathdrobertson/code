# https://financetrain.com/best-python-librariespackages-finance-financial-data-scientists/
# see https://nixos.org/nixos/packages.html to search for more
with import <nixpkgs> {};

# with pkgs;

let
  my-python-packages = python-packages: with python-packages; [
    pip
    pandas
    numpy
    matplotlib
    requests
    setuptools
    six
    # other python packages you want
  ]; 
  python-with-my-packages = python3.withPackages my-python-packages;
in

stdenv.mkDerivation {
  name = "heath-python";

  buildInputs = [python-with-my-packages nodejs];
  shellHook = ''
    pip install --upgrade mplfinance
    python3
  '';
}
