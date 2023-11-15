## miniscript-docker

#### Usage:

```
docker pull ghcr.io/bitcoincore-dev/miniscript-docker:latest
```
##### miniscript-docker

```
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/bitcoincore-dev/miniscript-docker/master/miniscript-docker)" -
```

![miniscript-docker-usage](https://github.com/bitcoincore-dev/miniscript-docker/assets/152159/16d58c91-35e1-41d4-9ec4-38b2eb9adf8c)


##### miniscript-docker build

```
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/bitcoincore-dev/miniscript-docker/master/miniscript-docker)" - build
```

![miniscript-docker-build](https://github.com/bitcoincore-dev/miniscript-docker/assets/152159/6de78217-51c5-4aed-9497-48586429db92)

##### miniscript-docker examples

```
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/bitcoincore-dev/miniscript-docker/master/miniscript-docker)" - examples
```

![miniscript-docker-examples](https://github.com/bitcoincore-dev/miniscript-docker/assets/152159/a9b7e0d6-bcdf-4e86-abe7-0af4dd3b887e)

<hr>

Go to [the Miniscript website](http://bitcoin.sipa.be/miniscript/).

This repository contains a C++ implementation of Miniscript and a number of
related things:
* The core Miniscript module ([cpp](bitcoin/script/miniscript.cpp), [h](bitcoin/script/miniscript.h)) together with a number of [dependencies](bitcoin/) based on
  the Bitcoin Core source code.
* A policy to Miniscript compiler ([cpp](compiler.cpp), [h](compiler.h)).
* Javascript wrappers for the website ([cpp](js_bindings.cpp)).
* The project website ([.html](index.html)).
