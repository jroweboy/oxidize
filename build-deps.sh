#!/bin/sh
RUST_VERSION=`rustc --version | awk "/host:/ { print \\$2 }"`

##
# Rust Openssl bindings are used by both rust-http and the secure cookie session implementation right now
# (I needed an HMAC binding and didn't feel like pulling in another dependency on rust crypto)
rm -rf target
mkdir -p target/$RUST_VERSION/lib

cd rust-openssl
make clean
./configure
make
cd ../target/$RUST_VERSION/lib 
ln -s ../../../rust-openssl/build/libopenssl* .
cd ../../../

##
# I want to make this pull either the dependencies for Mongrel2 or rusthttp depending on what
# you want. Till then though, I think its easier to pull both :p
cd rust-http
make clean
./configure
make
cd ../target/$RUST_VERSION/lib 
ln -s ../../../rust-http/build/libhttp* .
cd ../../../

cd rust-zmq
rm libz*
rustc --crate-type=rlib,dylib src/zmq/lib.rs
cd ../target/$RUST_VERSION/lib 
ln -s ../../../rust-zmq/libz* .
cd ../../../
