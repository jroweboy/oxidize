RUSTFLAGS ?= -O -Z debug-info
http_files=\
		      http/lib.rs \
		      http/buffer.rs \
		      http/common.rs \
		      http/generated/read_method.rs \
		      http/generated/status.rs \
		      $(wildcard http/headers/*.rs) \
		      $(wildcard http/client/*.rs) \
		      $(wildcard http/server/*.rs) \
		      http/memstream.rs \
		      http/method.rs \
		      http/rfc2616.rs

http: $(http_files)
	rustc $(RUSTFLAGS) --dylib --rlib http/lib.rs --out-dir=build

server: main.rs
	rustc $(RUSTFLAGS) main.rs -o build/main -L build/