cmake_minimum_required(VERSION 3.25)

project(cxx_demos)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
#set(CMAKE_CXX_FLAGS "-Wall -Werror -Wextra -Wpedantic -Wconversion -Wsign-conversion -Wfloat-equal")

set(CMAKE_EXECUTABLE_SUFFIX .wasm)
set(CMAKE_SYSTEM_PROCESSOR wasm32)
set(triple wasm32)

add_library(${CMAKE_PROJECT_NAME} INTERFACE)

# Include fwwasm header
add_subdirectory(../fwwasm fwwasm)
target_link_libraries(${PROJECT_NAME} INTERFACE fwwasm)

# Enable safety checks
target_compile_options(${PROJECT_NAME} INTERFACE -Wall -Werror -Wextra -Wpedantic -Wconversion -Wsign-conversion -Wfloat-equal -Wold-style-cast)
add_compile_options(-Wall -Werror -Wextra -Wpedantic -Wconversion -Wsign-conversion -Wfloat-equal -Wold-style-cast)
# WASM specific optimizations. 
# ERROR_ON_UNDEFINED_SYMBOLS = we need this to ignore import_module and import_name attribute unknown errors
# INITIAL_MEMORY = We only have 128KB of memory we can work with (2 pages)
# GLOBAL_BASE = This is the memory allocated for globals and strings. Defaults to 1024.
# --stack-first = We want to put the stack first as stack overflows should be easier to spot this way.
# -s = Strip debug symbols
target_link_options(${PROJECT_NAME} INTERFACE -s -Wl,--no-entry -Wl,--export-all -sERROR_ON_UNDEFINED_SYMBOLS=0 -sINITIAL_MEMORY=128KB -sGLOBAL_BASE=4096 --stack-first)
add_link_options(-s -Wl,--no-entry -Wl,--export-all -sERROR_ON_UNDEFINED_SYMBOLS=0 -sINITIAL_MEMORY=128KB -sGLOBAL_BASE=4096 --stack-first)

# Add all the examples below
add_subdirectory(src/radio)
add_subdirectory(src/simple_led)
add_subdirectory(src/rainbow_leds)