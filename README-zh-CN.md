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
})
```

# 反射

反射 API 返回普通 Lua table，而不是 Rust userdata wrapper：

```lua
local desc = pool:message("com.mikai233.Player")

print(desc.full_name)

for _, field in ipairs(desc.fields) do
    print(field.name, field.number, field.type, field.cardinality)
end
```

动态 message userdata 可用于按字段操作：

```lua
local msg = pool:new("com.mikai233.Player")

msg:set("id", "2347239423213")
msg:set("nickname", "mikai233")

print(msg:has("nickname"))
print(msg:get("id"))

local bytes = msg:encode()
local table_value = msg:to_table()
```

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
