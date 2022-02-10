# Copyright (c) Microsoft Corporation.
# Licensed under the MIT license.

export PREFIX ?= $(HOME)

export PKG_CONFIG_PATH ?= $(shell find $(PREFIX)/lib/ -name '*pkgconfig*' -type d)
export LD_LIBRARY_PATH ?= $(shell find $(PREFIX)/lib/ -name '*x86_64-linux-gnu*' -type d)
export CONFIG_PATH ?= $(HOME)/config.yaml

export TIMEOUT ?= 30

#===============================================================================
# Directories
#===============================================================================

export SRCDIR = $(CURDIR)/src
export BINDIR = $(CURDIR)/bin
export LIBDIR = $(CURDIR)/lib

#===============================================================================
# Toolchain Configuration
#===============================================================================

# Rust Toolchain
export BUILD ?= --release
export CARGO ?= $(HOME)/.cargo/bin/cargo

#===============================================================================

export DRIVER ?= $(shell [ ! -z "`lspci | grep -E "ConnectX-[4,5]"`" ] && echo mlx5 || echo mlx4)

#===============================================================================

all:
	$(CARGO) build $(BUILD) --features=$(DRIVER) $(CARGO_FLAGS)

clean: demikernel-clean

demikernel-tests:
	cd $(SRCDIR) && \
	$(CARGO) build --tests $(BUILD) --features=$(DRIVER) $(CARGO_FLAGS)

demikernel-clean:
	rm -rf target &&  \
	$(CARGO) clean && \
	rm -f Cargo.lock

#===============================================================================

test: test-catnip

test-catnip:
	cd $(SRCDIR) && \
	sudo -E LD_LIBRARY_PATH="$(LD_LIBRARY_PATH)" timeout $(TIMEOUT) $(CARGO) test $(BUILD) --features=$(DRIVER) $(CARGO_FLAGS) -p catnip-libos -- --nocapture $(TEST)

test-catnap:
	cd $(SRCDIR) && \
	sudo -E LD_LIBRARY_PATH="$(LD_LIBRARY_PATH)" timeout $(TIMEOUT) $(CARGO) test $(BUILD) $(CARGO_FLAGS) -p catnap-libos -- --nocapture $(TEST)