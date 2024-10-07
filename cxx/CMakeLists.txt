cmake_minimum_required(VERSION 3.25)
include(FetchContent)

SET(EXECUTABLE_SUFFIX ".wasm")

project(cxx_demos)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(${CMAKE_PROJECT_NAME} INTERFACE)

# Include fwwasm header
FetchContent_Declare(
    fwwasm
    GIT_REPOSITORY https://github.com/freewili/fwwasm.git
    GIT_TAG db333d6
)
FetchContent_MakeAvailable(fwwasm)

# Compiler and linker option variables
set(NORMAL_COMPILER_ARGS -Wall -Werror -Wextra -Wpedantic -Wconversion -Wsign-conversion -Wfloat-equal -Wold-style-cast)
set(WASM_COMPILER_ARGS --target=wasm32-unknown-wasi -O3 -flto)
set(WASM_LINKER_ARGS -Wl,--no-entry -Wl,--lto-O3 -Wl,-z,stack-size=102400)

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