#!/bin/bash

rm -rf artifacts/

cmake -S . -B artifacts/
cmake --build artifacts --parallel
./artifacts/cxx-cmake-example
