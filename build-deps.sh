#!/bin/sh

cd rust-http
./configure
make
cd ../
mkdir -p target/`rustc --version | awk "/host:/ { print \\$2 }"`/lib
# OSX is missing the relative path option so this may not work on Macs?
cd target/`rustc --version | awk "/host:/ { print \\$2 }"`/lib 
ln -s ../../../rust-http/build/libhttp-2cee9fa1-0.1-pre.* .
cd ../../../
make lib
