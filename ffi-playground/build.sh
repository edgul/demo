#!/bin/bash 
# run with source to update the LD_LIBRARY_PATH in the caller shell:
# source build.sh

# build dynlib
cd dynlib
cargo build --release

# build statlib
cd ../statlib
cargo build --release

# build and cpp and link
cd ..
g++ main.cpp statlib/target/release/libstatlib.a \
  -Ldynlib/target/release \
  -ldynlib \
  -o a.out

# tell the loader where to find dynamic libs 
# only do this if it's not already there
[[ ":$LD_LIBRARY_PATH:" == *":./dynlib/target/release:"* ]] \
  && echo "loader already knows" \
  || export LD_LIBRARY_PATH=./dynlib/target/release:$LD_LIBRARY_PATH  # Linux
