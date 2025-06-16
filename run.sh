#!/bin/bash

mkdir -p target

gcc -o main main.c -Wall -Werror -O3

mv main target/

./target/main $1