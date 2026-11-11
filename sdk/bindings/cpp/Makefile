SDK ?= /opt/clunkyard/sdk
NODE ?= node
CXX ?= c++
BUILD ?= $(TMPDIR)/clunkyard-cpp-binding

.PHONY: install test
install:
	$(NODE) install.cjs "$(SDK)"

test:
	mkdir -p $(BUILD)
	$(CXX) -std=c++20 -O2 -Wall -Wextra -Werror -Wno-unused-function -Wno-missing-field-initializers -I$(SDK)/include -I$(SDK)/bindings/cpp test_conformance.cpp $(SDK)/lib/libclunkyard.a -lpthread -ldl -lm -o $(BUILD)/conformance
	cd ../../.. && node sdk/bindings/conformance.js $(BUILD)/conformance
