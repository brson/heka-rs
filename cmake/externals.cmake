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
    GIT_TAG 6e315d1c00c49832b503a9cfa70b73a770e92c24
    CMAKE_ARGS ${SANDBOX_ARGS}
    INSTALL_DIR ${PROJECT_PATH}
    )
