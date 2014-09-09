-- This Source Code Form is subject to the terms of the Mozilla Public
-- License, v. 2.0. If a copy of the MPL was not distributed with this
-- file, You can obtain one at http://mozilla.org/MPL/2.0/.

require "string"

local tests = {
-- key, expected value
{"Type"             , "type"                                    },
{"Logger"           , "logger"                                  },
{"Payload"          , "payload"                                 },
{"EnvVersion"       , "envversion"                              },
{"Hostname"         , "hostname"                                },
{"Uuid"             , "f47ac10b-58cc-4372-a567-0e02b2c3d479"    },
{"Timestamp"        , 999                                       },
{"Severity"         , 4                                         },
{"Pid"              , 23                                        },
{"raw"              , "payload"                                 },
{"Fields[test]"     , "foo"                                     },
{"Fields[widget]"   , 222                                       },
{"Fields[nil]"      , nil                                       },
{"invalid"          , nil                                       },
}

local fields = {
-- key, field index, array index, expected value
{"Fields[test]" , 0, 0, "foo"},
{"Fields[test]" , 0, 1, "bar" },
{"Fields[test]" , 0, 2, nil   },
{"Fields[nil]"  , 0, 0, nil   },
{"bogus"        , 0, 0, nil   },
{"Fields[test]" , 2, 0, nil   },
{"Fields[test]" , 1, 0, "foo1"},
{"Fields[test]" , 1, 1, "bar1"},
}

function process_message()
    for i,v in ipairs(tests) do
        local f = read_message(v[1])
        if f ~= v[2] then
            error(string.format("%s expected:%s received:%s", v[1], tostring(v[2]), tostring(f)), 0)
        end
    end
    for i,v in ipairs(fields) do
        local f = read_message(v[1], v[2], v[3])
        if f ~= v[4] then
            error(string.format("%s[%d][%d] expected:%s received:%s", v[1], v[2], v[3], tostring(v[4]), tostring(f)), 0)
        end
    end
    return 0
end

function timer_event(ns)
    if read_message("Type") ~= nil then
        error("read_message should return nil")
    end
end
