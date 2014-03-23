cmake_minimum_required(VERSION 2.8)
project(test NONE)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake")
find_package(rustc)
find_package(rustdoc)
include(Rust)
find_package(PCRE)

set(RUSTC_FLAGS "-L${CMAKE_BINARY_DIR}/lib")
set(RUSTDOC_FLAGS "-L${CMAKE_BINARY_DIR}/lib")
get_filename_component(PCRE_LIB_LOC ${PCRE_LIBRARIES} PATH)

# Create the codegen binary, which we will use to generate other things.

##get_rust_deps(rust-http/src/codegen/main.rs HTTP_CODEGEN_DEPS)
##rust_crate(rust-http/src/codegen/main.rs
##           TARGET_NAME HTTP_CODEGEN
##           DEPENDS "${HTTP_CODEGEN_DEPS}")
##add_custom_target(HTTP_CODEGEN_TARGET
##                  ALL
##                  DEPENDS ${HTTP_CODEGEN_FULL_TARGET})
##
### I can't figure out how to get the actual output..
### get_property(CODEGEN_EXEC TARGET HTTP_CODEGEN_TARGET PROPERTY LOCATION)
##set(CODEGEN_EXEC "${HTTP_CODEGEN_ARTIFACTS}")
##
##
##add_custom_command( OUTPUT "${CMAKE_BINARY_DIR}/http/status.rs"
##					COMMAND "${CODEGEN_EXEC}" "status.rs" "${CMAKE_BINARY_DIR}"
##					DEPENDS HTTP_CODEGEN_TARGET
##					COMMENT "Generating HTTP Statuses"
##					VERBATIM)
##add_custom_command( OUTPUT "${CMAKE_BINARY_DIR}/http/read_method.rs"
##					COMMAND "${CODEGEN_EXEC}" "read_method.rs" "${CMAKE_BINARY_DIR}"
##					DEPENDS HTTP_CODEGEN_TARGET
##					COMMENT "Generating HTTP Read Method"
##					VERBATIM)
##
##add_custom_target(	rust_http
##					COMMAND make
##					WORKING_DIRECTORY "${CMAKE_SOURCE_DIR}/rust-http/")



# get_cmake_property(_variableNames VARIABLES)
# foreach (_variableName ${_variableNames})
#     message(STATUS "${_variableName}=${${_variableName}}")
# endforeach()


#get_rust_deps(rust-http/src/http/lib.rs HTTP_DEPS)
#
#rust_crate(rust-http/src/http/lib.rs
#           TARGET_NAME HTTP
#           DESTINATION lib
#           DEPENDS "${HTTP_DEPS}"
#           OTHER_RUSTC_FLAGS --crate-type dylib --crate-type rlib)
#

add_custom_target(
   rust_pcre
   COMMAND make
   WORKING_DIRECTORY "${CMAKE_SOURCE_DIR}/rust-pcre"
)
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

foreach( file_i ${libs})
    add_custom_command(
    TARGET file_i
    COMMAND ${CMAKE_COMMAND}
    ARGS -E copy ${file_i} "${CMAKE_BINARY_DIR}/lib/"
)
endforeach( file_i )
set(RUSTC_FLAGS "-L${CMAKE_SOURCE_DIR}/rust-pcre/lib"
                "-L${CMAKE_SOURCE_DIR}/rust-mustache/build"
                "-L${CMAKE_SOURCE_DIR}/rust-http/build"
                "-L${CMAKE_BINARY_DIR}"
                "-L${PCRE_LIB_LOC}"
              )

rust_crate_auto(src/oxidize.rs
           TARGET_NAME OXIDIZE
           DEPENDS rust_pcre rust_mustache rust_http ${libs}
           OTHER_RUSTC_FLAGS --crate-type dylib --crate-type rlib)

add_custom_target(oxidize_target
					ALL
					DEPENDS ${OXIDIZE_FULL_TARGET} rust_pcre rust_mustache rust_http)


add_dependencies(oxidize_target rust_pcre rust_mustache rust_http)

#--- example


rust_crate(example/hello_world/index.rs
            TARGET_NAME HELLO_WORLD
            DEPENDS ${OXIDIZE_FULL_TARGET}
            )
add_custom_target(hello_world_target
            ALL
            DEPENDS ${OXIDIZE_FULL_TARGET} ${HELLO_WORLD_FULL_TARGET})