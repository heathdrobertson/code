with import <nixpkgs> {};

# with pkgs;

stdenv.mkDerivation {
  name = "javascript-code";

  buildInputs = [python37 nodejs];
  shellHook = ''
    npm install -g @vue/cli-service-global mocha
  '';
}
