# lua-protobuf-rs

此项目基于 Rust 的 [protobuf](https://github.com/stepancheg/rust-protobuf) 实现，为 Lua 提供运行时 Protobuf 支持。

核心目标是：**不需要把 `.proto` 编译成静态 Lua/Rust 文件**，Lua 可以在运行时加载 schema、查询反射信息，并进行 encode/decode。

可以做到：

- 运行时加载 `.proto` 文件、proto 字符串或 descriptor set
- 将 Lua table 编码成 Protobuf 二进制字符串
- 将 Protobuf 二进制字符串解码成 Lua table
- 在 Lua 中查询 message、enum、service、field 等 schema 信息
- 生成 LuaLS/EmmyLua 类型提示文件

# 使用

## 直接解析 proto 字符串

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

## 解析 proto 文件

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

也可以使用 descriptor set：

```lua
local pool = pb.load_descriptor_set("proto.pb")
```

# 编解码语义

默认行为面向 Lua 运行时使用：

- 未设置的字段解码后为 `nil`，不会自动填默认值
- `int64` / `uint64` 默认使用字符串，避免 Lua number 精度损失
- `bytes` 使用 Lua binary string
- enum 默认使用名字，例如 `"ONLINE"`
- encode 遇到未知字段默认报错
- 一次 encode 输入里同时设置同一个 `oneof` 的多个字段默认报错
- 写出二进制前会递归校验 proto2 `required` 字段

可以通过选项调整：

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

可以动态 pack/unpack `google.protobuf.Any`：

```lua
local any = pool:pack_any("com.mikai233.Player", {
    id = "42",
})

local type_name, player = pool:unpack_any(any)
print(type_name, player.id)
```

当 pure Rust parser 无法解析 well-known type import 时，会自动 fallback 到
内置的 `protoc`。因此运行时加载包含 `google/protobuf/any.proto`、
`timestamp.proto`、`duration.proto`、`wrappers.proto` 等 import 的 schema
不需要依赖系统安装的 `protoc`。

如果只想校验 table 而不关心编码后的二进制，可以使用 `validate`：

```lua
local ok, err = pool:validate("com.mikai233.Player", {
    attrs = { hp = {} },
})

if not ok then
    print(err)
    -- com.mikai233.Player.attrs["hp"]: value Table cannot be cast to int64
end
```

# 反射

反射 API 返回普通 Lua table，而不是 Rust userdata wrapper：

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

动态 message userdata 可用于按字段操作：

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

从更新 schema 解析到的 unknown fields 会在 dynamic message 再次 encode 时保留。
可以用 `msg:unknown_fields()` 检查，也可以用
`msg:clear_unknown_fields(number?)` 在转发前丢弃它们。

# proto 代码提示

使用 `gen_lua` 生成 LuaLS/EmmyLua 注解文件：

```lua
pool:gen_lua("proto")
```

示例输出：

```lua
---@class com_mikai233_LoginResponse
---@field player? com_mikai233_Player
local com_mikai233_LoginResponse = { }
```

# Descriptor 导出

运行时加载后的 schema 可以导出，后续直接加载缓存：

```lua
pool:write_descriptor_set("proto.pb")
local cached = pb.load_descriptor_set("proto.pb")

pool:write_file_descriptors("proto-pb")
local cached_files = pb.load_descriptor_set("proto-pb")
```

也可以直接获得内存中的 descriptor set 二进制字符串：

```lua
local descriptor_set_bytes = pool:descriptor_set()
```

# xLua 插件集成

设置环境变量 `LUA_LIB_NAME`、`LUA_LIB` 为 xLua 的头文件目录以及 xLua 的库名，然后重新编译此项目即可。
注意 xLua 的版本一定要和此项目的 Lua 版本对应。

```csharp
[DllImport("lua_protobuf_rs", CallingConvention = CallingConvention.Cdecl)]
public static extern int luaopen_lua_protobuf_rs(System.IntPtr L);

[MonoPInvokeCallback(typeof(LuaDLL.lua_CSFunction))]
public static int LoadProtobufRs(System.IntPtr L)
{
    return luaopen_lua_protobuf_rs(L);
}
```

# 编译

```shell
cargo build --release
```

对于不同的 Lua 版本，修改 `Cargo.toml` 中的 feature 后重新编译即可。

## 交叉编译

- 编译到 Linux：`cross build --target x86_64-unknown-linux-gnu --release`
- 编译到 Android：`cross build --target armv7-linux-androideabi --release`
