# Change the second -L to the location of oxidize if you are adapting this for your own code
# Also it needs the location of rust-http... I need to find a way to copy the rust-http build 
# so that you only need one directory to link against or something
LINKFLAGS ?= -L rust-mustache/build -L ../../build -L ../../rust-http/build
RUSTFLAGS ?= -O
# uncomment this for a debug build
#RUSTFLAGS ?= -g

hello_files=\
	index.rs

all: hello_world

hello_world: rust-mustache/build/mustache bin/hello_world

bin/hello_world: $(hello_files)
	mkdir -p bin/
	rustc $(RUSTFLAGS) $(LINKFLAGS) index.rs --out-dir bin/

rust-mustache/build/mustache: rust-mustache/Makefile
	$(MAKE) -C rust-mustache/
	touch rust-mustache/build/mustache

clean:
	rm -rf bin