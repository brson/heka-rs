-- This Source Code Form is subject to the terms of the Mozilla Public
-- License, v. 2.0. If a copy of the MPL was not distributed with this
-- file, You can obtain one at http://mozilla.org/MPL/2.0/.

function process_message()
    local p = read_message("Pid")
    if p == 0 then
        write_message() -- too few args
    elseif p == 1 then
         write_message("Fields[bogus]", 0, "count", 0, 0, 0) -- too many args
    elseif p == 2 then
        write_message(nil, "test") -- wrong field type
    elseif p == 3 then
        write_message("test", nil) -- wrong value type
    elseif p == 4 then
        write_message("test", 1, nil, -1) -- negative field index
    elseif p == 5 then
        write_message("test", 1, nil, 0, -1) -- negative array index
    elseif p == 6 then
        write_message("test", "test") -- unknown field
    elseif p == 7 then
        write_message("Fields[test]", 1, nil, 1) -- invalid field index
    elseif p == 8 then
        write_message("Fields[test]", 1, nil, 0, 1) -- invalid array index
    elseif p == 9 then
        write_message("Type", 1) -- wrong value type
    elseif p == 10 then
        write_message("Logger", 1) -- wrong value type
    elseif p == 11 then
        write_message("Payload", 1) -- wrong value type
    elseif p == 12 then
        write_message("EnvVersion", 1) -- wrong value type
    elseif p == 13 then
        write_message("Hostname", 1) -- wrong value type
    elseif p == 14 then
        write_message("Timestamp", "1") -- wrong value type
    elseif p == 15 then
        write_message("Severity", true) -- wrong value type
    elseif p == 16 then
        write_message("Pid", true) -- wrong value type
    elseif p == 17 then
        write_message("Fields[Bool]", true)
        write_message("Fields[Bool]", "1", nil, 0, 1) -- type mis-match
    elseif p == 18 then
        write_message("Uuid", true)
    elseif p == 19 then
        write_message("Uuid", "foobar")
    end
    -- todo test string Timestamps
    return 0
end
