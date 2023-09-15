# lua-protobuf-rs

此项目使用的是rust的[protobuf](https://github.com/stepancheg/rust-protobuf)实现，编写的rust到lua的protobuf绑定

可以做到在运行时加载proto文件，将protobuf二进制数据解析成lua table、将lua table编码成二进制数据以及protobuf在lua下的反射功能

绑定的API可以到[这里](https://docs.rs/protobuf/latest/protobuf/)
查看，绑定了几个比较关键的struct， `FileDescriptor` `MessageDescriptor`
`FieldDescriptor` `EnumDescriptor`，这几个结构其实就表示了proto的文件、消息、以及字段、枚举的信息

# 使用

## lua里直接解析proto

```lua
--- @type LuaProtoc
local luaProtoc = require("lua_protobuf_rs")

---@language "protobuf"
local proto = [[
syntax="proto3";

message Player{
  int64 id = 1;
  int64 world_id = 2;
  string nickname = 3;
  int32 exp = 4;
}

message LoginRequest{
  int64 id = 1;
  int64 world_id = 2;
}

message LoginResponse{
  Player player = 1;
}
]]

local protoc = luaProtoc.compile_proto(proto)

local player = {
    id = 2347239423213,
    world_id = 234872389,
    nickname = "mikai233",
    exp = 22000,
}

local player_bytes = protoc:encode("Player", player)
local decode_player = protoc:decode("Player", player_bytes)
print(decode_player.id)

local login_response_bytes = protoc:encode("LoginResponse", {})
local decode_login_response = protoc:decode("LoginResponse", login_response_bytes)
print(decode_login_response.player.id)
```

## lua里解析proto文件

- player.proto

```protobuf
syntax = "proto3";

package com.mikai233;

message Player{
  int64 id = 1;
  int64 world_id = 2;
  string nickname = 3;
  int32 exp = 4;
}
```

- login.proto

```protobuf
syntax = "proto3";
import "player.proto";

package com.mikai233;

message LoginRequest{
  int64 id = 1;
  int64 world_id = 2;
}

message LoginResponse{
  Player player = 1;
}
```

```lua
--- @type LuaProtoc
local luaProtoc = require("lua_protobuf_rs")

local protos = luaProtoc.list_protos({ "proto" })

local protoc = luaProtoc.compile_file(protos, { "proto" })

local player = {
    id = 2347239423213,
    world_id = 234872389,
    nickname = "mikai233",
    exp = 22000,
}

local player_bytes = protoc:encode("com.mikai233.Player", player)
local decode_player = protoc:decode("com.mikai233.Player", player_bytes)
print(decode_player.id)

local login_response_bytes = protoc:encode("com.mikai233.LoginResponse", {})
local decode_login_response = protoc:decode("com.mikai233.LoginResponse", login_response_bytes)
print(decode_login_response.player.id)
```

## 反射

更多的反射API请查看文档

```lua
--- @type LuaProtoc
local luaProtoc = require("lua_protobuf_rs")

local protos = luaProtoc.list_protos({ "proto" })

local protoc = luaProtoc.compile_file(protos, { "proto" })

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
```

## proto代码提示

可以使用`gen_lua`
来生成lua的proto模板文件，来获得更好的代码编写体验，本项目是基于[EmmyLua](https://github.com/EmmyLua/IntelliJ-EmmyLua)
插件做的注解提示

```lua
---@class LoginRequest
---@field id number
---@field world_id number
local LoginRequest

---@class LoginResponse
---@field player Player
local LoginResponse
```

# 编译

得益于Cargo，编译变得非常简单，只需要安装Rust，然后执行`cargo build --release`就可以得到当前平台的库文件

对于不同的Lua版本，只需要修改`Cargo.toml`中的`default`字段重新编译即可

## 交叉编译

如果需要编译到不同平台，可以使用[交叉编译](https://github.com/cross-rs/cross)，需要Docker，操作难度很小

- 编译到Linux：`cross build --target x86_64-unknown-linux-gnu --release`
- 编译到Android `cross build --target armv7-linux-androideabi --release`

# 注意事项

对于非`oneof`类型的字段，在将二进制消息解析成table时，如果该字段没有设置值，都会给一个默认值，而对于`oneof`类型的字段，如果都没有设置过值，
那么这些字段都将不存在，也就是说`oneof`的所有字段只可能存在一个或者都不存在
