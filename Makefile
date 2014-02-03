LINKFLAGS ?= -L build/
RUSTFLAGS ?= -O -Z debug-info

example_files=\
			example/hello_world/index.rs

oxidize_files=\
			src/oxidize.rs\
			src/renderer.rs\
			src/route.rs

all: oxidize examples

examples: $(example_files)
	mkdir -p build/examples/hello_world/
	rustc $(RUSTFLAGS) $(LINKFLAGS) -o build/examples/hello_world/hello_world example/hello_world/index.rs
	cp -R example/hello_world/templates build/examples/hello_world/

# TODO: This rebuilds everytime even if there is no change.
# I just happen to suck at Makefiles
oxidize: $(oxidize_files)
	rustc $(RUSTFLAGS) $(LINKFLAGS) --dylib --rlib src/oxidize.rs --out-dir build/

.PHONY: clean

clean:
	rm -rf build/examples build/liboxidize