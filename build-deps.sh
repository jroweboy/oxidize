#!/bin/sh
RUST_VERSION=`rustc --version | awk "/host:/ { print \\$2 }"`

##
# Rust Openssl bindings are used by both rust-http and the secure cookie session implementation right now
# (I needed an HMAC binding and didn't feel like pulling in another dependency on rust crypto)

cd rust-openssl
./configure
make

##
# I want to make this pull either the dependencies for Mongrel2 or rusthttp depending on what
# you want. Till then though, I think its easier to pull both :p
cd rust-http
./configure
make
cd ../
mkdir -p target/$RUST_VERSION/lib
# OSX is missing the relative path option so this may not work on Macs?
cd target/$RUST_VERSION/lib 
ln -s ../../../rust-http/build/libhttp-2cee9fa1-0.1-pre.* .
cd ../../../
make lib
