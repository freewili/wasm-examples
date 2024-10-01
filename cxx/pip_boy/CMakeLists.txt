# $ rm -R build && emcmake cmake -B build && cmake --build build && wasm2wat build/pip_boy.wasm
cmake_minimum_required(VERSION 3.25)

project(pip_boy VERSION 1.0.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(CMAKE_EXECUTABLE_SUFFIX .wasm)
set(CMAKE_SYSTEM_PROCESSOR wasm32)
set(triple wasm32)

add_compile_options(-Wall -Werror -Wextra -Wpedantic -Wconversion -Wsign-conversion -Wfloat-equal)
# Strip debug symbols
add_link_options(-s)
add_compile_options(-Os -flto)
add_link_options(-Wl,--no-entry -Wl,--export-all -sERROR_ON_UNDEFINED_SYMBOLS=0 -sINITIAL_MEMORY=128KB -sGLOBAL_BASE=4096 --stack-first)
add_executable(pip_boy src/demo.cpp)
include_directories(include)

add_subdirectory(../../fwwasm ../../fwwasm)
include_directories(../../fwwasm/include)
