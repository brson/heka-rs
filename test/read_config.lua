-- This Source Code Form is subject to the terms of the Mozilla Public
-- License, v. 2.0. If a copy of the MPL was not distributed with this
-- file, You can obtain one at http://mozilla.org/MPL/2.0/.

require "string"

local tests = {
{"string"   , "widget"  },
{"int64"    , 99        },
{"double"   , 99.123    },
{"bool"     , true      },
{"nil"      , nil       },
}

for i,v in ipairs(tests) do
    local f = read_config(v[1])
    if f ~= v[2] then
        error(string.format("%s expected:%s received:%s", v[1], tostring(v[2]), tostring(f)), 0)
    end
end

