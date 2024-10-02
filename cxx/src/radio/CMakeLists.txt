cmake_minimum_required(VERSION 3.25)

project(cxx_radio VERSION 1.0.0 LANGUAGES CXX)

add_executable(${PROJECT_NAME} radio.cpp)

target_link_libraries(${PROJECT_NAME} PUBLIC fwwasm)