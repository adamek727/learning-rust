cmake_minimum_required(VERSION 3.10)
project(matlib C)
add_library(matlib STATIC math.c)
install(TARGETS matlib DESTINATION .)