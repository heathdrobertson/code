# Plutus in NixOS Container
__Plutus In Container__

- TODO
https://github.com/input-output-hk/plutus.git

```bash
docker run -it \
    --volumes-from nix \
    --volume $(pwd):/home \
    --workdir /home \
    --name plutus \
    nixos/nix nix build -f default.nix haskell.projectPackages.language-plutus-core
```

