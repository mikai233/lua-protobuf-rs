# lua-protobuf-rs

Available languages: [English](README.md) | [中文](README-zh-CN.md)

This project provides runtime Protobuf support for Lua, built on top of Rust’s
[protobuf](https://github.com/stepancheg/rust-protobuf) implementation.

The goal is: **Lua can use Protobuf without generating static Lua or Rust files from `.proto` files**.
Schemas can be loaded at runtime, reflected in Lua, and used for encode/decode.

It supports:

- Loading `.proto` files, inline proto strings, or descriptor sets at runtime
- Encoding Lua tables into Protobuf binary strings
- Decoding Protobuf binary strings into Lua tables
- Querying message, enum, service, and field schema information in Lua
- Generating LuaLS/EmmyLua type hint files

# Usage

## Parse proto directly in Lua

```lua
local pb = require("lua_protobuf_rs")

---@language "protobuf"
local proto = [[
syntax = "proto3";

message Player {
  int64 id = 1;
  int64 world_id = 2;
  string nickname = 3;
  int32 exp = 4;
}

message LoginResponse {
  Player player = 1;
}
]]

local pool = pb.load_proto(proto)

local player_bytes = pool:encode("Player", {
    id = "2347239423213",
    world_id = "234872389",
    nickname = "mikai233",
    exp = 22000,
})

local player = pool:decode("Player", player_bytes)
print(player.id)
```

## Parse proto files in Lua

```lua
local pb = require("lua_protobuf_rs")

local files = pb.list_protos({ "proto" })
local pool = pb.load({
    files = files,
    includes = { "proto" },
})

local bytes = pool:encode("com.mikai233.LoginResponse", {
    player = {
        id = "2347239423213",
        world_id = "234872389",
        nickname = "mikai233",
        exp = 22000,
    },
})

local message = pool:decode("com.mikai233.LoginResponse", bytes)
print(message.player.nickname)
```

Descriptor sets are supported too:

```lua
local pool = pb.load_descriptor_set("proto.pb")
```

# Codec Semantics

Default behavior is optimized for Lua runtime usage:

- Unset fields decode to `nil`; defaults are not filled automatically
- `int64` / `uint64` values use strings by default to avoid Lua number precision loss
- `bytes` values use Lua binary strings
- enums use names by default, for example `"ONLINE"`
- unknown fields during encode are rejected by default
- multiple fields from the same `oneof` in one encode input are rejected by default
- proto2 `required` fields are validated recursively before writing bytes

Options can override these defaults:

```lua
local message = pool:decode("com.mikai233.Player", bytes, {
    defaults = true,
    int64 = "integer",
    bytes = "table",
    enum = "number",
})

local bytes = pool:encode("com.mikai233.Player", message, {
    unknown = "ignore",
    oneof = "last",
})
```

Use `validate` when you want to check a table without keeping the encoded bytes:

```lua
local ok, err = pool:validate("com.mikai233.Player", {
    attrs = { hp = {} },
})

if not ok then
    print(err)
    -- com.mikai233.Player.attrs["hp"]: value Table cannot be cast to int64
end
```

# Reflection

Reflection APIs return plain Lua tables instead of Rust userdata wrappers:

```lua
local desc = pool:message("com.mikai233.Player")

print(desc.full_name)
print(desc.fields_by_name.id.type)

for _, field in ipairs(desc.fields) do
    print(field.name, field.number, field.type, field.cardinality)
end

local player_field = desc.fields_by_name.player
if player_field and player_field.resolved_type.kind == "message" then
    print(player_field.resolved_type.full_name)
end
```

Dynamic message userdata is available when field-by-field mutation is useful:

```lua
local msg = pool:new("com.mikai233.Player", {
    id = "2347239423213",
})

msg:set("nickname", "mikai233")
msg:merge({ email = "dev@example.com" })

print(msg:has("nickname"))
print(msg:get("id"))
print(msg:which_oneof("contact"))

local ok, err = msg:validate()
local unknown_fields = msg:unknown_fields()

local bytes = msg:encode()
local table_value = msg:to_table()
```

Unknown fields parsed from newer schemas are preserved when a dynamic message is
encoded again. Use `msg:unknown_fields()` for inspection and
`msg:clear_unknown_fields(number?)` when forwarding should drop them.

# Proto Code Hints

Use `gen_lua` to generate LuaLS/EmmyLua annotation files:

```lua
pool:gen_lua("proto")
```

Example output:

```lua
---@class com_mikai233_LoginResponse
---@field player? com_mikai233_Player
local com_mikai233_LoginResponse = { }
```

# Descriptor Export

Runtime schemas can be exported and loaded again later:

```lua
pool:write_descriptor_set("proto.pb")
local cached = pb.load_descriptor_set("proto.pb")

pool:write_file_descriptors("proto-pb")
local cached_files = pb.load_descriptor_set("proto-pb")
```

For in-memory use:

```lua
local descriptor_set_bytes = pool:descriptor_set()
```

# xLua Integration

Set the environment variables `LUA_LIB_NAME` and `LUA_LIB` to point to the xLua header directory and library name, then
recompile this project.
Make sure the xLua version matches the Lua version used in this project.

```csharp
[DllImport("lua_protobuf_rs", CallingConvention = CallingConvention.Cdecl)]
public static extern int luaopen_lua_protobuf_rs(System.IntPtr L);

[MonoPInvokeCallback(typeof(LuaDLL.lua_CSFunction))]
public static int LoadProtobufRs(System.IntPtr L)
{
    return luaopen_lua_protobuf_rs(L);
}
```

# Build

```shell
cargo build --release
```

For different Lua versions, adjust the relevant Cargo features and rebuild.

# Cross-Compilation

- Build for Linux: `cross build --target x86_64-unknown-linux-gnu --release`
- Build for Android: `cross build --target armv7-linux-androideabi --release`
