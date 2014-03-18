# my beautiful handwritten makefile. Bask in its glorious complexity

# TODO: replace this with a much cooler build system. At least it works
# but if anyone cares it would be nice to use cmake or something
# Google/Github search for rust cmake, since I know they are out there somewhere
LINKFLAGS ?= -L lib 
RUSTFLAGS ?= --crate-type=dylib,rlib 

example_hello_world=\
			example/hello_world/index.rs

benchmarks=\
			benchmarks/techempower/techempower.rs

oxidize_files=\
			src/oxidize.rs\
			src/renderer.rs\
			src/route/mod.rs\
			src/route/regexrouter.rs\
			src/conf.rs

OXIDIZE_LIB = build/liboxidize-a719aadf-0.0.so

all: oxidize examples benchmarks

# ugly hack to get the libraries here and keep from having to recompile
# them. I add a file with the name of the make rule. clean will clear this though
lib/mustache: rust-mustache/Makefile
	$(MAKE) -C rust-mustache/
	mkdir -p lib
	touch lib/mustache
# I originally used the -r option but not everyones machine had that :S
	#cd lib && ln -sf ../rust-mustache/build/libmustache*.rlib .
	#cd lib && ln -sf ../rust-mustache/build/libmustache*.so .
	cp rust-mustache/build/libmustache*.so lib/
	cp rust-mustache/build/libmustache*.rlib lib/

lib/http: rust-http/Makefile
	$(MAKE) -C rust-http/
	mkdir -p lib
	touch lib/http
	#cd lib && ln -sf ../rust-http/build/libhttp*.rlib .
	#cd lib && ln -sf ../rust-http/build/libhttp*.so .
	cp rust-http/build/libhttp*.rlib lib/
	cp rust-http/build/libhttp*.so lib/

lib/pcre: rust-pcre/Makefile
	$(MAKE) -C rust-pcre/
	mkdir -p lib
	touch lib/pcre
	#cd lib && ln -sf ../rust-pcre/lib/libpcre*.rlib .
	#cd lib && ln -sf ../rust-pcre/lib/libpcre*.so .
	cp rust-pcre/lib/libpcre*.rlib lib/
	cp rust-pcre/lib/libpcre*.so lib/

# Main program
oxidize: lib/mustache lib/http lib/pcre $(OXIDIZE_LIB)

$(OXIDIZE_LIB): $(oxidize_files)
	mkdir -p build/
	rustc $(RUSTFLAGS) $(LINKFLAGS) src/oxidize.rs --out-dir build/

# Example program
examples: $(example_hello_world)
	mkdir -p build/examples/hello_world/
	rustc $(LINKFLAGS) -L build -o \
		build/examples/hello_world/hello_world $(example_hello_world)
	cp -R example/hello_world/templates build/examples/hello_world/

# Benchmark program for http://www.techempower.com/benchmarks/
benchmarks: $(benchmarks)
	mkdir -p build/benchmarks/techempower/
	rustc $(LINKFLAGS) -L build -o \
		build/benchmarks/techempower/techempower $(benchmarks)

run:
	cd build/examples/hello_world && ./hello_world

run-benchmark:
	cd build/benchmarks/techempower && ./techempower

run-gdb:
	cd build/examples/hello_world && gdb ./hello_world

# Other stuff
.PHONY: all clean examples run run-gdb

clean:
	rm -rf build lib

clean-all:
	rm -rf build lib
	$(MAKE) -C rust-http/ clean
	$(MAKE) -C rust-pcre/ clean
	$(MAKE) -C rust-mustache/ clean


#########
## Unused vars
##

OBJ_FLAGS ?= --emit=link --out-dir $(OBJ_DIR)

# make sure there is not a trailing space after these two
OBJ_DIR ?= build/obj
EXAMPLE_OBJ ?= build/obj/example/

example_obj=\
			$(EXAMPLE_OBJ)/hello_world/index.o

oxidize_obj=\
			$(OBJ_DIR)/oxidize.bc\
			$(OBJ_DIR)/renderer.bc\
			$(OBJ_DIR)/route.bc

LIBHTTP = lib/libhttp.rlib
LIBMUSTACHE = lib/libmustache.rlib
LIBPCRE = lib/libpcre.rlib

#########
## These targets wouldn't work since rust wouldn't output an obj file?
##

$(EXAMPLE_OBJ)/hello_world/index.bc: example/hello_world/index.rs
	mkdir -p $(OBJ_DIR)
	rustc $(OBJ_FLAGS) example/hello_world/index.rs

#########
## These targets didn't work since I renamed the link to the library 
## so rust failed to use the libraries.

#all: $(LIBMUSTACHE) $(LIBHTTP) $(LIBPCRE) oxidize examples

# Dependancies. touch the Makefile on an individual 
# project to rebuild it or just call make clean-all
$(LIBHTTP): rust-http/Makefile
	$(MAKE) -C rust-http/
	mkdir -p lib
	ln -srf rust-http/build/libhttp*.rlib $(LIBHTTP)
	ln -srf rust-http/build/libhttp*.so lib/libhttp.so

$(LIBPCRE): rust-pcre/Makefile
	$(MAKE) -C rust-pcre/
	mkdir -p lib
	ln -srf rust-pcre/lib/libpcre*.rlib $(LIBPCRE)

$(LIBMUSTACHE): rust-mustache/Makefile
	$(MAKE) -C rust-mustache/
	mkdir -p lib
	ln -srf rust-mustache/build/libmustache*.rlib $(LIBMUSTACHE)
	ln -srf rust-mustache/build/libmustache*.so lib/libmustache.so
