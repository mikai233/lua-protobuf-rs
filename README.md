# lua-protobuf-rs

此项目使用的是[rust的protobuf实现](https://github.com/stepancheg/rust-protobuf)，编写的rust到lua的protobuf绑定

可以做到在运行时加载proto文件，将protobuf二进制数据解析成lua table以及将lua table编码成二进制数据以及protobuf在lua下的反射功能

绑定的api可以到[这里](https://docs.rs/protobuf/latest/protobuf/)
查看，绑定了几个比较关键的struct， `FileDescriptor` `MessageDescriptor`
`FieldDescriptor` `EnumDescriptor`，这几个结构其实就表示了proto的文件、消息、以及字段、枚举的信息

# 使用

```protobuf
syntax = "proto3";

message Student{
  string name = 1;
  uint32 age = 2;
}
```

```lua
local protoc = require("lua_protobuf_rs")
local protos = protoc.list_protos({ "." })

protoc = protoc.new(protos, { "." })
local bytes = protoc:encode("Student", { name = "mikai233" })
local student = protoc:decode("Student", bytes)
print(student.name)
```

# 编译

## 交叉编译

