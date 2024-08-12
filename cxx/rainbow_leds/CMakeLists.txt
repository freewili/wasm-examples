cmake_minimum_required(VERSION 3.25)

project(led_demo VERSION 1.0.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(CMAKE_EXECUTABLE_SUFFIX .wasm)

add_compile_options(-Wall -Wextra -Wpedantic -Werror -Wconversion -Wsign-conversion -Wfloat-equal)
# Strip debug symbols
add_link_options(-s)

add_executable(led_demo src/demo.cpp)
include_directories(include)
