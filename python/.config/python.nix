# see https://nixos.org/nixos/packages.html to search for more
with import <nixpkgs> {};

# with pkgs;

let
  my-python-packages = python-packages: with python-packages; [
    jupyterlab
    pandas
    numpy
    requests
    six
    # other python packages you want
  ]; 
  python-with-my-packages = python38.withPackages my-python-packages;
in

stdenv.mkDerivation {
  name = "heath-python";

  buildInputs = [python-with-my-packages nodejs];
  shellHook = ''
    jupyter labextension install @jupyterlab/toc
    jupyter labextension install jupyterlab_vim
    jupyter lab --allow-root --ip=0.0.0.0 --port=8080
  '';
}
