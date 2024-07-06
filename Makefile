.DEFAULT_GOAL := build

version.mk: Cargo.toml
	echo VERSION := $(shell grep version $< | head -n1 | cut -d \" -f 2 | tr -d -) > $@
include version.mk

image: Cargo.toml
	curl -L 'https://github.com/BelaPlatform/bela-image-builder/releases/download/v$(VERSION)/bela_image_v$(VERSION).img.xz' | \
		xz --decompress > $@

.PHONY: mount
mnt/:; mkdir $@
mount: image | mnt/
	mount | grep --quiet $(PWD)/mnt || mount -o ro \
		"`losetup --show --find --read-only --partscan $<`p2" \
		mnt

.PHONY: prepare-cross
prepare-cross: sysroot.tar.gz

sysroot.tar.gz: | mount
	tar --create --file $@ --auto-compress --directory mnt .

.PHONY: build

$(HOME)/.cargo/bin/bindgen:
	cargo install bindgen-cli

CLANG_C_ARGS := --target=armv7-unknown-linux-gnueabihf \
	-isysroot=mnt \
	-isystem mnt/usr/xenomai/include \
	-isystem mnt/root/Bela/include
CLANG_CXX_ARGS := $(CLANG_C_ARGS) \
	-stdlib++-isystem mnt/usr/include/c++/6 \
	-isystem mnt/usr/include/arm-linux-gnueabihf/c++/6 \
	-x c++ -std=c++11 # from Bela's Makefile

build: src/bindings.rs
src/bindings.rs: $(HOME)/.cargo/bin/bindgen | mount
	$< mnt/root/Bela/include/Bela.h \
		--allowlist-type 'Bela.*' \
		--allowlist-function 'Bela_.*' \
		--allowlist-function 'rt_.*printf' \
		--allowlist-var '(BELA|DEFAULT|MAX)_.*' \
		-- $(CLANG_C_ARGS) \
		> $@

build: src/midi_bindings.rs
src/midi_bindings.rs: $(HOME)/.cargo/bin/bindgen | mount
	$< mnt/root/Bela/libraries/Midi/Midi_c.h \
		--allowlist-function 'Midi_.*' \
		-- $(CLANG_C_ARGS) \
		> $@

build: src/trill_bindings.rs
src/trill_bindings.rs: $(HOME)/.cargo/bin/bindgen | mount
	$< mnt/root/Bela/libraries/Trill/Trill.h \
		--allowlist-function 'Trill.*' \
		--opaque-type 'std::.*' \
		-- $(CLANG_CXX_ARGS) \
		> $@
