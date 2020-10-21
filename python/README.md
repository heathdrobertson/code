# Python 
__Python Programming Language Tools & Code__


## Quick Start


## 1. Python `app` or `module`

```bash
cd app
```
```bash
export container_name=python
```
```bash
docker run -it \
--name ${CONTAINER_NAME} \
--volumes-from nix \
--volume $(pwd):/home/ci \
--workdir /home/ci \
heathdrobertson/nix nix-shell
```

## 2. Python via `Jupyter Lab` 

```bash
cd lab
```
```bash
export container_name=py-lab
```
```bash
docker run -it \
--name ${CONTAINER_NAME} \
--volumes-from nix \
--volume $(pwd):/mnt\
--workdir /mnt \
--env JUPYTER_ENABLE_LAB=yes \
--env HOME=/mnt \
--publish 8888:8888 \
heathdrobertson/nix nix-shell /mnt/.config/build.nix
```

- Module: ```tools``` 
    - Web Scraping Recipie Ingredients
    - [Google API Access](tools/) from the Command Line

