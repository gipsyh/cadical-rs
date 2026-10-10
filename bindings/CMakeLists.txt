cmake_minimum_required(VERSION 3.10)
project(bindings VERSION 1.0.0)
include(ExternalProject)

enable_language(C CXX)
set(CMAKE_CXX_STANDARD 11)

add_library(bindings binding.cpp)
target_include_directories(bindings PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/../include)

install(TARGETS bindings
        LIBRARY DESTINATION lib
        PUBLIC_HEADER DESTINATION include
        )