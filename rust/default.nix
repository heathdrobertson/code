with import <nixpkgs> {};

let
  sources = {
    "chalk-4.1.0" = {
      name = "chalk";
      packageName = "chalk";
      version = "4.1.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/chalk/-/chalk-4.1.0.tgz";
        sha512 = "qwx12AxXe2Q5xQ43Ac//I6v5aXTipYrSESdOgzrN+9XjgEpyjpKuvSGaN4qE93f7TQTlerQQ8S+EQ0EyDoVL1A==";
      };
    };
    "figlet-1.5.0" = {
      name = "figlet";
      packageName = "figlet";
      version = "1.5.0";
      src = fetchurl {
        url = "https://registry.npmjs.org/figlet/-/figlet-1.5.0.tgz";
        sha512 = "ZQJM4aifMpz6H19AW1VqvZ7l4pOE9p7i/3LyxgO2kp+PO/VcDYNqIHEMtkccqIhTXMKci4kjueJr/iCQEaT/Ww==";
      };
    };
  };
in

dependencies = [
  sources."chalk-4.1.0"
  sources."figlet-1.5.0"
];

stdenv.mkDerivation {
  name = "rust-env";
  buildInputs = [
    # Example Run-time Additional Dependencies
    cargo
    rustc 
    rustup
    cargo-generate
    openssl
    dependencies
  ];

  # Set Environment Variables
  RUST_BACKTRACE = 1;
  shellHook = ''
    rustc --version
  '';
}
