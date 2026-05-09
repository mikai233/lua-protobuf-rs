---@meta lua_protobuf_rs

---@class LuaProtobufRs
local pb = {}

---@class LuaProtoLoadConfig
---@field proto? string
---@field files? string[]
---@field includes? string[]
---@field descriptor_set? string

---@class LuaProtoCodecOptions
---@field defaults? boolean
---@field int64? "string"|"integer"
---@field bytes? "string"|"table"
---@field enum? "name"|"number"
---@field unknown? "error"|"ignore"
---@field oneof? "error"|"last"

---@param config LuaProtoLoadConfig
---@return LuaProtoPool
function pb.load(config) end

---@param inputs string[]
---@param includes string[]
---@return LuaProtoPool
function pb.load_files(inputs, includes) end

---@param proto string
---@return LuaProtoPool
function pb.load_proto(proto) end

---@param path string
---@return LuaProtoPool
function pb.load_descriptor_set(path) end

---@param paths string[]
---@return string[]
function pb.list_protos(paths) end

---@class LuaProtoPool
local LuaProtoPool = {}

---@param message_full_name string
---@param message table
---@param options? LuaProtoCodecOptions
---@return string binary
function LuaProtoPool:encode(message_full_name, message, options) end

---@param message_full_name string
---@param bytes string binary
---@param options? LuaProtoCodecOptions
---@return table
function LuaProtoPool:decode(message_full_name, bytes, options) end

---@param message_full_name string
---@param message table
---@param options? LuaProtoCodecOptions
---@return boolean ok
---@return string? err
function LuaProtoPool:validate(message_full_name, message, options) end

---@param message_full_name string
---@param message? table
---@param options? LuaProtoCodecOptions
---@return LuaDynamicMessage
function LuaProtoPool:new(message_full_name, message, options) end

---@param message_full_name string
---@param bytes string binary
---@return LuaDynamicMessage
function LuaProtoPool:decode_message(message_full_name, bytes) end

---@return LuaFileSchema[]
function LuaProtoPool:files() end

---@param name string
---@return LuaFileSchema?
function LuaProtoPool:file(name) end

---@return LuaMessageSchema[]
function LuaProtoPool:messages() end

---@param name string
---@return LuaMessageSchema?
function LuaProtoPool:message(name) end

---@return LuaEnumSchema[]
function LuaProtoPool:enums() end

---@param name string
---@return LuaEnumSchema?
function LuaProtoPool:enum(name) end

---@return LuaServiceSchema[]
function LuaProtoPool:services() end

---@param name string
---@return LuaServiceSchema?
function LuaProtoPool:service(name) end

---@param path string
function LuaProtoPool:gen_lua(path) end

---@return string binary
function LuaProtoPool:descriptor_set() end

---@param path string
function LuaProtoPool:write_descriptor_set(path) end

---@param path string
function LuaProtoPool:write_file_descriptors(path) end

---@class LuaDynamicMessage
local LuaDynamicMessage = {}

---@return string
function LuaDynamicMessage:type_name() end

---@return LuaMessageSchema
function LuaDynamicMessage:descriptor() end

---@return LuaUnknownField[]
function LuaDynamicMessage:unknown_fields() end

---@param field_name string
---@return boolean
function LuaDynamicMessage:has(field_name) end

---@param oneof_name string
---@return string?
function LuaDynamicMessage:which_oneof(oneof_name) end

---@param field_name string
---@param options? LuaProtoCodecOptions
---@return any
function LuaDynamicMessage:get(field_name, options) end

---@param field_name string
---@param value any
---@param options? LuaProtoCodecOptions
function LuaDynamicMessage:set(field_name, value, options) end

---@param field_name? string
function LuaDynamicMessage:clear(field_name) end

---@param oneof_name string
function LuaDynamicMessage:clear_oneof(oneof_name) end

---@param number? integer
function LuaDynamicMessage:clear_unknown_fields(number) end

---@param message table
---@param options? LuaProtoCodecOptions
function LuaDynamicMessage:merge(message, options) end

---@param options? LuaProtoCodecOptions
---@return table
function LuaDynamicMessage:to_table(options) end

---@return boolean ok
---@return string? err
function LuaDynamicMessage:validate() end

---@return string binary
function LuaDynamicMessage:encode() end

---@class LuaUnknownField
---@field number integer
---@field wire_type "varint"|"fixed32"|"fixed64"|"length_delimited"
---@field value string|integer

---@class LuaFileSchema
---@field kind "file"
---@field name string
---@field package string
---@field syntax "proto2"|"proto3"
---@field dependency string[]
---@field public_dependency integer[]
---@field messages LuaMessageSchema[]
---@field messages_by_name table<string,LuaMessageSchema>
---@field enums LuaEnumSchema[]
---@field enums_by_name table<string,LuaEnumSchema>
---@field services LuaServiceSchema[]
---@field services_by_name table<string,LuaServiceSchema>

---@class LuaMessageSchema
---@field kind "message"
---@field name string
---@field full_name string
---@field file string
---@field package string
---@field relative_name string
---@field map_entry boolean
---@field fields LuaFieldSchema[]
---@field fields_by_name table<string,LuaFieldSchema>
---@field fields_by_json_name table<string,LuaFieldSchema>
---@field fields_by_number table<integer,LuaFieldSchema>
---@field oneofs LuaOneofSchema[]
---@field oneofs_by_name table<string,LuaOneofSchema>
---@field nested_messages LuaMessageSchema[]
---@field nested_messages_by_name table<string,LuaMessageSchema>
---@field nested_enums LuaEnumSchema[]
---@field nested_enums_by_name table<string,LuaEnumSchema>

---@class LuaFieldSchema
---@field kind "field"
---@field name string
---@field json_name string
---@field number integer
---@field full_name string
---@field containing_message string
---@field required boolean
---@field repeated boolean
---@field map boolean
---@field optional boolean
---@field proto3_optional boolean
---@field oneof string?
---@field default string?
---@field cardinality "singular"|"repeated"|"map"
---@field type? string
---@field type_name? string
---@field key_type? string
---@field key_type_name? string
---@field value_type? string
---@field value_type_name? string
---@field resolved_type LuaResolvedType

---@class LuaResolvedType
---@field kind string
---@field name? string
---@field full_name? string
---@field relative_name? string
---@field map_entry? boolean
---@field item? LuaResolvedType
---@field key? LuaResolvedType
---@field value? LuaResolvedType

---@class LuaOneofSchema
---@field name string
---@field full_name string
---@field synthetic boolean
---@field containing_message string
---@field fields LuaFieldSchema[]
---@field field_names string[]
---@field fields_by_name table<string,LuaFieldSchema>

---@class LuaEnumSchema
---@field kind "enum"
---@field name string
---@field full_name string
---@field relative_name string
---@field values LuaEnumValueSchema[]
---@field values_by_name table<string,LuaEnumValueSchema>
---@field values_by_number table<integer,LuaEnumValueSchema>
---@field default_value LuaEnumValueSchema

---@class LuaEnumValueSchema
---@field name string
---@field full_name string
---@field number integer

---@class LuaServiceSchema
---@field kind "service"
---@field name string
---@field full_name string
---@field methods LuaMethodSchema[]
---@field methods_by_name table<string,LuaMethodSchema>

---@class LuaMethodSchema
---@field name string
---@field input_type string
---@field input LuaMessageSchema
---@field output_type string
---@field output LuaMessageSchema
---@field client_streaming boolean
---@field server_streaming boolean

return pb
