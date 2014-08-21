if(NOT PROTOBUF_EXECUTABLE)
    message(FATAL_ERROR "Google protocol buffers 'protoc' must be installed, message.proto has been modified and needs to be regenerated.")
endif()

execute_process(
COMMAND ${PROTOBUF_EXECUTABLE} --rust_out=. --plugin=${RUST_PLUGIN} message.proto;
WORKING_DIRECTORY ${MESSAGE_DIR}
)
execute_process(
COMMAND ${CMAKE_COMMAND} -E rename message.rs pb.rs
WORKING_DIRECTORY ${MESSAGE_DIR}
)
