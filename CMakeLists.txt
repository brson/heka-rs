# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

cmake_minimum_required(VERSION 2.8 FATAL_ERROR)
project(heka_rs)
set(CPACK_PACKAGE_DESCRIPTION_SUMMARY "Heka Rust Experimentation")
set(CPACK_PACKAGE_VERSION_MAJOR 0)
set(CPACK_PACKAGE_VERSION_MINOR 0)
set(CPACK_PACKAGE_VERSION_PATCH 1)

set(PROJECT_PATH "${CMAKE_BINARY_DIR}/${PROJECT_NAME}")
set(CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake")
include(CTest)
include(externals)
find_package(Rust 0.12.0 REQUIRED)
find_package(Protobuf 2.3 QUIET)

set(MESSAGE_PROTO_OUT "${CMAKE_SOURCE_DIR}/src/message.rs")
add_custom_command(
OUTPUT ${MESSAGE_PROTO_OUT}
COMMAND ${CMAKE_COMMAND} -DPROTOBUF_EXECUTABLE="${PROTOBUF_EXECUTABLE}" -DRUST_PLUGIN="${PROJECT_PATH}/bin/protoc-gen-rust" -DMESSAGE_DIR="${CMAKE_SOURCE_DIR}/src" -P "${CMAKE_SOURCE_DIR}/cmake/message_proto.cmake"
DEPENDS rust-protobuf "${CMAKE_SOURCE_DIR}/src/message.proto"
COMMENT "Built ${MESSAGE_PROTO_OUT}"
)

add_custom_target(heka_rs ALL
${RUST_EXECUTABLE} -O -L "${PROJECT_PATH}/lib" -o heka_test "${CMAKE_SOURCE_DIR}/src/heka_rs.rs"
DEPENDS lua_sandbox rust-protobuf ${MESSAGE_PROTO_OUT})

add_custom_target(unit_tests ALL
${RUST_EXECUTABLE} -L "${PROJECT_PATH}/lib" -o unit_tests --test "${CMAKE_SOURCE_DIR}/src/heka_rs.rs"
DEPENDS heka_rs)

add_test(unit_tests unit_tests)
