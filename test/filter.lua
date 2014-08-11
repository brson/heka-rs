function process_message()
    if read_message("Type") ~= "type" then error(tostring(read_message("Type"))) end
--    if read_message("Logger") ~= "logger" then return -2 end
--    if read_message("Payload") ~= "payload" then return -3 end
--    if read_message("EnvVersion") ~= "envversion" then return -4 end
--    if read_message("Hostname") ~= "hostname" then return -5 end
--    if read_message("Uuid") ~= "uuid" then return -6 end
--    if read_message("Timestamp") ~= 999 then return -7 end
--    if read_message("Severity") ~= 4 then return -8 end
--    if read_message("Pid") ~= 23 then return -9 end
--    if read_message("raw") ~= "payload" then return -10 end

    return 0
end
