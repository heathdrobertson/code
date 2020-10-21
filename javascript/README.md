# Javascript
__JavaScript Practice Code &amp; Tools__


## Quick Start

### To creat the container:

```bash
export CONTAINER_NAME=javascript
```

```bash
docker run -it \
--name ${CONTAINER_NAME} \
--volumes-from nix \
--volume $(pwd):/home/ci \
heathdrobertson/nix:latest
```
