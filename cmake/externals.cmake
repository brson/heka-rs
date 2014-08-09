# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

include(ExternalProject)

set_property(DIRECTORY PROPERTY EP_BASE "${CMAKE_BINARY_DIR}/ep_base")

set(PLUGIN_LOADER ${PLUGIN_LOADER} "github.com/mozilla-services/heka/sandbox/plugins")
set(SANDBOX_ARGS -DCMAKE_BUILD_TYPE=${CMAKE_BUILD_TYPE} -DCMAKE_INSTALL_PREFIX=${PROJECT_PATH} -DADDRESS_MODEL=${ADDRESS_MODEL} -DLUA_JIT=off --no-warn-unused-cli)
externalproject_add(
    lua_sandbox
    GIT_REPOSITORY https://github.com/mozilla-services/lua_sandbox.git
    GIT_TAG 3da8912fe29e3fdd865b7f476d79cbe300d6af74
    CMAKE_ARGS ${SANDBOX_ARGS}
    INSTALL_DIR ${PROJECT_PATH}
    )

externalproject_add(
    rust-protobuf
    GIT_REPOSITORY https://github.com/stepancheg/rust-protobuf.git
    GIT_TAG fd06c5cf3db936d998d9a1b723c0eb7865fab530
    BINARY_DIR "${CMAKE_BINARY_DIR}/ep_base/Source/rust-protobuf/src"
    BUILD_COMMAND ./rebuild.sh
    CONFIGURE_COMMAND ""
    INSTALL_COMMAND ${CMAKE_COMMAND} -E make_directory "${PROJECT_PATH}/lib/"&& ${CMAKE_COMMAND} -E make_directory "${PROJECT_PATH}/bin/" && ${CMAKE_COMMAND} -E copy libprotobuf.rlib "${PROJECT_PATH}/lib/" && ${CMAKE_COMMAND} -E copy protoc-gen-rust "${PROJECT_PATH}/bin/"
    UPDATE_COMMAND "" # comment out to enable updates
)
