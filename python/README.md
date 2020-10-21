# Python 
__Python Programming Language Tools & Code__


## Quick Start

```bash
docker run -d \
--name py-code \
--volumes-from=nix \
-v=$(pwd):/mnt \
-w=/mnt \
-e JUPYTER_ENABLE_LAB=yes \
-e HOME=/mnt \
-p=8888:8888 \
nixos/nix nix-shell
```

- Module: ```tools``` 
    - Web Scraping Recipie Ingredients
    - [Google API Access](tools/) from the Command Line

