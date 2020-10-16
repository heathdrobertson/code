# Plutus in NixOS Container
__Plutus In Container__

```bash
docker run -it \
    --volumes-from nix \
    --volumes-from config \
    -v $(pwd):/home \
    -w /home \
    --name plutus \
    nixos/nix nix build -f default.nix haskell.projectPackages.language-plutus-core
```

