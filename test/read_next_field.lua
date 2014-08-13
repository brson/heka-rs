-- This Source Code Form is subject to the terms of the Mozilla Public
-- License, v. 2.0. If a copy of the MPL was not distributed with this
-- file, You can obtain one at http://mozilla.org/MPL/2.0/.

require "string"

local tests = {
-- type, name, vaule, representation, count
{0  , "foo"     , "bar"         , "test", 1     },
{1  , "bytes"   , "data"        , ""    , 1     },
{2  , "int"     , 999           , ""    , 2     },
{3  , "double"  , 99.9          , ""    , 1     },
{4  , "bool"    , true          , ""    , 1     },
{0  , "foo"     , "alternate"   , ""    , 1     },
{4  , "false"   , false         , ""    , 1     },
{nil, nil       , nil           , nil   , nil   }
}

function process_message()
    for i,v in ipairs(tests) do
        local t, name, value, representation, count = read_next_field()
        if not(t == v[1] and name == v[2] and value == v[3] and representation == v[4] and count == v[5]) then
            error(string.format("test: %d expected: t:%s name:%s value:%s representation:%s count:%s received: t:%s name:%s value:%s representation:%s count:%s", i, tostring(v[1]), tostring(v[2]), tostring(v[3]), tostring(v[4]), tostring(v[5]), tostring(t), tostring(name), tostring(value), tostring(representation), tostring(count)), 0)
            return -1
        end
    end
    return 0
end

function timer_event(ns)
    read_next_field("too many args")
end
