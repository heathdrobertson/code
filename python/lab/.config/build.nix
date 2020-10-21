# https://financetrain.com/best-python-librariespackages-finance-financial-data-scientists/
# see https://nixos.org/nixos/packages.html to search for more
with import <nixpkgs> {};

# with pkgs;

let
  my-python-packages = python-packages: with python-packages; [
    pip
    jupyterlab
    pandas
    numpy
    matplotlib
    requests
    setuptools
    six
    krakenx
    python-binance
    # other python packages you want
  ]; 
  python-with-my-packages = python3.withPackages my-python-packages;
in

stdenv.mkDerivation {
  name = "heath-python";

  buildInputs = [python-with-my-packages nodejs];
  shellHook = ''
    pip install --upgrade mplfinance
    npm i @jupyterlab/toc
    npm i jupyterlab_vim
    jupyter labextension enable @jupyterlab/toc
    jupyter labextension enable jupyterlab_vim
    jupyter lab --allow-root --no-browser --ip=0.0.0.0 --port=8888
  '';
}
