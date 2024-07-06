# bela-sys

C & C++ API bindings for
the [Bela musical interface microprocessor](https://bela.io/).

## Environment

The [stable Bela](https://github.com/BelaPlatform/bela-image-builder) is running
a [very old version of Debian](https://www.debian.org/releases/stretch/),
which makes it difficult to have a correct development environment:

- you probably don't run an ARMv7-A toolchain on a Xenomai kernel
- modern `bindgen` is not running on Bela because it needs clang >= 6
- theses old libc and friends are very likely to not exist on your system
- [libbela](https://github.com/BelaPlatform/Bela)
  isn't buildable outside of the board

That being said, your project only needs to link to the actual library,
not to generate the bindings. It is thus a separate and optional process.

### Build

As no cross image was made for Bela, one need to build it.
To do that, you'll need

- [QEMU User space emulator](https://qemu.eu/doc/latest/user/main.html)
  for ARMv7-A
- [`binfmt_misc`](https://en.wikipedia.org/wiki/Binfmt_misc)
  setup to run ARMv7-A with `/usr/bin/qemu-arm`
  - if that's not your path, change it in
    the [Makefile](Makefile) and [Dockerfile](Dockerfile)
- [cross](https://github.com/cross-rs/cross) for Cargo integration

The [Makefile](Makefile) takes care of having creating a tar of the image.
The rest is achieved via `cross`.

```sh
$ make image # get Bela image based on cargo's version
# make sysroot.tar.gz # mount image and tar it (requires root)
$ cross build --example hello # build whatever you want
```

## Generate bindings

If you want to build your own bindings

```sh
$ make image # get Bela image based on cargo's version
# make mount # mount image (requires root)
$ make build # generate bindings from mounted image
```
