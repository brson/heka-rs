#!/bin/bash
ROOT="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd $OUT_DIR
cmake -DCMAKE_BUILD_TYPE=release -DRUST_OUT_DIR=$OUT_DIR $ROOT
make
cp $OUT_DIR/lib/* $OUT_DIR
