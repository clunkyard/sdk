# The Python binding: make install SDK=/opt/clunkyard/sdk puts the package in
# $(SDK)/bindings/python; Python creates its own bytecode cache. A robot runs with
# PYTHONPATH=$(SDK)/bindings/python and no compiler. make test runs the
# installed package against every API vector.
SDK ?= /opt/clunkyard/sdk
NODE ?= node
CORE_LIBRARY ?= $(firstword $(wildcard $(SDK)/lib/libclunkyard.so.2 $(SDK)/lib/libclunkyard.2.dylib $(SDK)/lib/clunkyard.dll))
PYTHON ?= python3

.PHONY: install test
install:
	$(NODE) install.cjs "$(SDK)"

# -P keeps this directory's copy off the path, so the test imports the installed one.
test:
	cd ../../.. && CLUNKYARD_LIB=$(CORE_LIBRARY) PYTHONPATH=$(SDK)/bindings/python \
	  node sdk/bindings/conformance.js $(PYTHON) -P sdk/bindings/python/test_conformance.py
	CLUNKYARD_LIB=$(CORE_LIBRARY) PYTHONPATH=$(SDK)/bindings/python $(PYTHON) -P test_loop.py
