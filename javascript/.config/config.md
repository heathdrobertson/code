# NixOS Docker Container Running a Nix Shell
__Development Environment__

1. In terminal window 1 of 3, run a Docker container:
```bash
docker run --rm --volumes-from=nix -it -v $(pwd):/app -w /app -p 4000:4000 nixos/nix nix-shell /app/.config/node.nix
```
```bash
npm init
```
