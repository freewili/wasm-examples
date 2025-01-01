cmake_minimum_required(VERSION 3.0)
project(wasm_project)

set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -O1 -s")

add_executable(accel.wasm "accel.cpp")