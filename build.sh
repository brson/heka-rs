#!/bin/bash
ROOT=$PWD
cd $OUT_DIR

cmake -DCMAKE_BUILD_TYPE=release $ROOT
CMAKE_RET=$?
if [ $CMAKE_RET -ne 0 ]; then
    exit $CMAKE_RET
fi

make
MAKE_RET=$?
if [ $MAKE_RET -ne 0 ]; then
    exit $MAKE_RET
fi

cp $OUT_DIR/lib/* $OUT_DIR
exit $?
