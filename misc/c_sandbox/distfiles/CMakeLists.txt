cmake_minimum_required(VERSION 3.16.3)

project(dummy NONE)
set(CMAKE_C_COMPILER /usr/bin/clang-11 CACHE PATH "")
set(CMAKE_CXX_COMPILER /usr/bin/clang++-11 CACHE PATH "")
set(CMAKE_CXX_FLAGS "${CMAKE_C_FLAGS} -std=c++1z")

project(CSandbox)

find_package(LLVM REQUIRED CONFIG HINTS "/usr/lib/llvm-11/")

message(STATUS "Found LLVM ${LLVM_PACKAGE_VERSION}")
message(STATUS "Using LLVMConfig.cmake in: ${LLVM_DIR}")

include_directories(${LLVM_INCLUDE_DIRS})
add_definitions(${LLVM_DEFINITIONS})

add_library(CSandbox SHARED sandbox.cpp)
target_link_libraries(CSandbox ${llvm_libs})
