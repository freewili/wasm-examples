cmake_minimum_required(VERSION 3.25)

set(CMAKE_EXECUTABLE_SUFFIX ${EXECUTABLE_SUFFIX})

project(accel VERSION 1.0.0 LANGUAGES CXX)

add_executable(${PROJECT_NAME} accel.cpp)

target_link_libraries(${PROJECT_NAME} PUBLIC fwwasm)