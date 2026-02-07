cmake_minimum_required(VERSION 4.1)
project(SIRC C)

set(CMAKE_C_STANDARD 11)

add_executable(SIRC
        sirc-client/client.c
        sirc-server/server.c)
