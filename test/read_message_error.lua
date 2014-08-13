-- This Source Code Form is subject to the terms of the Mozilla Public
-- License, v. 2.0. If a copy of the MPL was not distributed with this
-- file, You can obtain one at http://mozilla.org/MPL/2.0/.

function process_message()
    local p = read_message("Pid")
    if p == 0 then
        read_message() -- too few args
    elseif p == 1 then
        read_message("x", 1, 2, 3) -- too many args
    elseif p == 2 then
        read_message({}) -- wrong type
    elseif p == 3 then
        read_message("x", -1, 0) -- negative field index
    elseif p == 4 then
        read_message("x", 1, -2) -- negative array index
    end
    return 0
end
