# Haskell
__Functional Programming__


Preparing for building smart contracts on Cardano's public blockchain network using
`Plutus` and `Marlowe`.


- [Plutus Playground](https://prod.playground.plutus.iohkdev.io/)
    - [Docs](https://docs.cardano.org/projects/plutus/en/latest/index.html)
    - [Tutorial](https://prod.playground.plutus.iohkdev.io/tutorial/)
- [Marlowe Playground](https://alpha.marlowe.iohkdev.io/#/)
    - [Tutorial](https://alpha.marlowe.iohkdev.io/tutorial/index.html)

## Quick Start

### Option 1 - Interactive Glasgow Haskell Compiler (GHCi)
__Using a Nix derivation in Docker Container__

- https://downloads.haskell.org/~ghc/latest/docs/html/users_guide/index.html

#### To creat the container:

```bash
export CONTAINER_NAME=<change_me>
```

```bash
docker run -it \
--name ${CONTAINER_NAME} \
--volumes-from nix \
--volume $(pwd):/home/ci \
heathdrobertson/nix:latest
```

```bash
Prelude> :cd hello/
Prelude> :load hello
Main> main
Main> :quit
```

Or from the `nix-shell`:
```bash
cd hello && ghc hello.hs && ./hello
```

#### To restart the container after a closed session.

```bash
docker start haskell
```

```bash
docker exec -it haskell nix-shell
```


### Option 2 - Jupyter Notebook - IHaskell (Haskell Kernel)

```bash
docker run -it \
--name ihaskell \
-p 8888:8888 \
--volume $(pwd):/home/jovyan/work \
--workdir /home/jovyan/work \
gibiansky/ihaskell:latest
```


## Learning Haskell Resources

- [Learn You a Haskell for Great Good](http://learnyouahaskell.com/)
- [Get Progamming with Haskell](https://livebook.manning.com/book/get-programming-with-haskell/chapter-1/)
- [Real World Haskell](http://book.realworldhaskell.org/read/)


