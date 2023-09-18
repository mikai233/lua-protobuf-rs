---
--- Generated by EmmyLua(https://github.com/EmmyLua)
--- Created by dream.
--- DateTime: 2023/9/15 15:09
---

--- @type LuaProtoc
local luaProtoc = require("lua_protobuf_rs")

local protos = luaProtoc.list_protos({ "proto" })

local protoc = luaProtoc.parse_files(protos, { "proto" })

local player_descriptor = protoc:message_descriptor_by_name("com.mikai233.Player")

for _, field in pairs(player_descriptor:fields()) do
    print("field name: " .. field:name() .. " number: " .. field:number())
end

print("====")

local login_response_descriptor = protoc:message_descriptor_by_name("com.mikai233.LoginResponse")

for _, field in pairs(login_response_descriptor:fields()) do
    local rt = field:runtime_field_type()
    if rt.singular then
        local singular = rt.singular
        print(singular.message:name())
    end
end
