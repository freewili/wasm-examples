cmake_minimum_required(VERSION 3.25)
include(FetchContent)

SET(EXECUTABLE_SUFFIX ".wasm")

project(cxx_demos)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(${CMAKE_PROJECT_NAME} INTERFACE)

# Use FetchContent_Declare example, we have the submodule already so no need here. Left as an example.
# # Include fwwasm header
# FetchContent_Declare(
#     fwwasm
#     GIT_REPOSITORY https://github.com/freewili/fwwasm.git
#     GIT_TAG 5ce1a060
# )
# FetchContent_MakeAvailable(fwwasm)

add_subdirectory(../fwwasm fwwasm)

# Compiler and linker option variables
set(NORMAL_COMPILER_ARGS -Wall -Werror -Wextra -Wpedantic -Wconversion -Wsign-conversion -Wfloat-equal -Wold-style-cast)
set(WASM_COMPILER_ARGS --target=wasm32-unknown-wasi -O3 -flto -mbulk-memory)
set(WASM_LINKER_ARGS
    "-Wl,--no-entry" # Specify we don't need main exported
    "-Wl,--export-all" # Export all symbols
    "-Wl,--lto-O3"
    "-Wl,-z,stack-size=49152" # Stack size: 48KB (matching Rust config)
    "-Wl,--initial-heap=0" # Heap should just fill in remaining that is left. See __heap_base and __head_end exports.
    "-Wl,--max-memory=65536" # Limit to 1 page (64KB) total memory (matching Rust config)
    "-Wl,--initial-memory=65536" # We only have 1 page (64KB) to work with on the Free-Wili
    "-Wl,--stack-first" # Place the stack first so its easier to find stack overflow issues.
    "-Wl,--strip-all" # Strip all debug symbols - wasm2wat is more useful without stripping.
)

# Compiler options
target_compile_options(${PROJECT_NAME} INTERFACE ${NORMAL_COMPILER_ARGS} ${WASM_COMPILER_ARGS})
add_compile_options(${NORMAL_COMPILER_ARGS} ${WASM_COMPILER_ARGS})
# Linker options
target_link_options(${PROJECT_NAME} INTERFACE ${WASM_LINKER_ARGS})
add_link_options(${WASM_LINKER_ARGS})

# Add all the examples under examples directory
message(STATUS "==========================================")
message(STATUS "Looking for examples to build in:")
message("${CMAKE_CURRENT_SOURCE_DIR}/examples")
message(STATUS "==========================================")
FILE(GLOB example_dirs ${CMAKE_CURRENT_SOURCE_DIR}/examples/*)
foreach(dir ${example_dirs})
    if(IS_DIRECTORY ${dir} AND EXISTS ${dir}/CMakeLists.txt)
        message(STATUS "Found example: ${dir}")
        add_subdirectory(${dir})
    endif()
endforeach()