cmake_minimum_required(VERSION 2.8)
project(test NONE)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake")
find_package(rustc)
find_package(rustdoc)
include(Rust)

set(RUSTC_FLAGS "-L${CMAKE_BINARY_DIR}/lib")
set(RUSTDOC_FLAGS "-L${CMAKE_BINARY_DIR}/lib")

add_custom_target(
   rust_mustache
   COMMAND make
   WORKING_DIRECTORY "${CMAKE_SOURCE_DIR}/rust-mustache"
)
add_custom_target(
   rust_http
   COMMAND make
   WORKING_DIRECTORY "${CMAKE_SOURCE_DIR}/rust-http"
)

#------------------------------------------------------------------

file(GLOB libs "${CMAKE_SOURCE_DIR}/rust-*/*/lib*.*")

set(RUSTC_FLAGS "-L${CMAKE_SOURCE_DIR}/rust-mustache/build"
                "-L${CMAKE_SOURCE_DIR}/rust-http/build"
                "-L${CMAKE_BINARY_DIR}"
              )

rust_crate_auto(src/oxidize.rs
           TARGET_NAME OXIDIZE
           DEPENDS rust_mustache rust_http ${libs}
           OTHER_RUSTC_FLAGS --crate-type dylib --crate-type rlib)

add_custom_target(oxidize_target
					ALL
					DEPENDS ${OXIDIZE_FULL_TARGET} rust_mustache rust_http)


add_dependencies(oxidize_target rust_mustache rust_http)

#--- example


rust_crate(example/hello_world/index.rs
            TARGET_NAME HELLO_WORLD
            DEPENDS ${OXIDIZE_FULL_TARGET}
            )
add_custom_target(hello_world_target
            ALL
            DEPENDS ${OXIDIZE_FULL_TARGET} ${HELLO_WORLD_FULL_TARGET})