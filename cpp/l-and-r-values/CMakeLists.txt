# Minimum required version of CMake
cmake_minimum_required(VERSION 3.10)

# Project name
project(proj)

# Set C++ standard
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -stdlib=libc++")
set(CMAKE_CXX_STANDARD_REQUIRED True)
set(CMAKE_CXX_COMPILER clang++)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Add source files
add_executable(proj main.cpp)
