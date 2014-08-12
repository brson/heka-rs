-- This Source Code Form is subject to the terms of the Mozilla Public
-- License, v. 2.0. If a copy of the MPL was not distributed with this
-- file, You can obtain one at http://mozilla.org/MPL/2.0/.

function process_message()
    if read_message("Type") ~= "type" then return -1 end
    if read_message("Logger") ~= "logger" then return -2 end
    if read_message("Payload") ~= "payload" then return -3 end
    if read_message("EnvVersion") ~= "envversion" then return -4 end
    if read_message("Hostname") ~= "hostname" then return -5 end
--    if read_message("Uuid") ~= "f47ac10b-58cc-4372-a567-0e02b2c3d479" then return -6 end
    if read_message("Timestamp") ~= 999 then return -7 end
    if read_message("Severity") ~= 4 then return -8 end
    if read_message("Pid") ~= 23 then return -9 end
    if read_message("raw") ~= "payload" then return -10 end

    return 0
end
