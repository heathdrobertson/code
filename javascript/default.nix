with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "nodeenv";
  buildInputs = [nodejs];
  shellHook = ''
    cd hello
    npm install
    node hello.js
  '';
}
