cmake_minimum_required(VERSION 3.15)
project(M1OEP)

set(CMAKE_CXX_STANDARD 17)

add_executable(M1OEP main.cpp board.h piece.h board.cpp piece.cpp game.cpp game.h)

# test executable

# add_executable(tests tests.cpp board.h piece.h board.cpp piece.cpp game.cpp game.h)
