# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

cmake_minimum_required(VERSION 2.8 FATAL_ERROR)
project(heka_rs)
set(CPACK_PACKAGE_DESCRIPTION_SUMMARY "Heka Rust Experimentation")
set(CPACK_PACKAGE_VERSION_MAJOR 0)
set(CPACK_PACKAGE_VERSION_MINOR 0)
set(CPACK_PACKAGE_VERSION_PATCH 1)

set(PROJECT_PATH "$ENV{OUT_DIR}")
set(CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake")
include(CTest)
include(externals)
# find_package(Rust 0.12.0 REQUIRED)
# find_package(Cargo 0.0.1 REQUIRED)
find_package(Protobuf 2.3 QUIET)

set(MESSAGE_PROTO_OUT "${CMAKE_SOURCE_DIR}/src/message/pb.rs")

add_custom_command(
OUTPUT ${MESSAGE_PROTO_OUT}
COMMAND ${CMAKE_COMMAND} -DPROTOBUF_EXECUTABLE="${PROTOBUF_EXECUTABLE}" -DRUST_PLUGIN="${PROJECT_PATH}/bin/protoc-gen-rust" -DMESSAGE_DIR="${CMAKE_SOURCE_DIR}/src/message" -P "${CMAKE_SOURCE_DIR}/cmake/message_proto.cmake"
DEPENDS rust-protobuf "${CMAKE_SOURCE_DIR}/src/message/message.proto"
COMMENT "Built ${MESSAGE_PROTO_OUT}"
)
