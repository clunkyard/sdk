SDK ?= /opt/clunkyard/sdk
RUSTC ?= rustc
CC ?= cc
BUILD ?= $(TMPDIR)/clunkyard-rust-binding
DEST = $(SDK)/bindings/rust
ifeq ($(shell uname -s),Darwin)
DYLIB_LINK_FLAGS = -C 'link-arg=-Wl,-rpath,$(SDK)/lib'
endif

.PHONY: install test
install:
	node install.cjs "$(SDK)" "$(RUSTC)"

test:
	mkdir -p "$(BUILD)"
	$(CC) -std=c17 -O2 -Wall -Wextra -Werror -Wno-unused-function -Dmain=clunkyard_vector_main "-I$(SDK)/include" -c test_conformance.c -o "$(BUILD)/vectors.o"
	$(RUSTC) --edition=2021 -O --extern "clunkyard=$(DEST)/libclunkyard.rlib" -L "native=$(SDK)/lib" -C "link-arg=$(BUILD)/vectors.o" $(DYLIB_LINK_FLAGS) -C link-arg=-lpthread -C link-arg=-ldl -C link-arg=-lm test_conformance.rs -o "$(BUILD)/conformance"
	cd ../../.. && node sdk/bindings/conformance.js "$(BUILD)/conformance"
