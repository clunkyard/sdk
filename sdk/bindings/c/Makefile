SDK ?= /opt/clunkyard/sdk
NODE ?= node
CC ?= cc
BUILD ?= $(TMPDIR)/clunkyard-c-binding

.PHONY: install test
install:
	$(NODE) install.cjs "$(SDK)"

test:
	mkdir -p $(BUILD)
	$(CC) -std=c17 -O2 -Wall -Wextra -Werror -Wno-unused-function -I$(SDK)/include test_conformance.c $(SDK)/lib/libclunkyard.a -lpthread -ldl -lm -o $(BUILD)/conformance
	cd ../../.. && node sdk/bindings/conformance.js $(BUILD)/conformance
