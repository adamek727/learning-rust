cmake_minimum_required(VERSION 3.13)
project(c_project C)

set(CMAKE_C_STANDARD 11)

link_directories(libs)

add_executable(c_project main.c)
target_link_libraries(c_project librust_lib.a pthread ${CMAKE_DL_LIBS})