use mlua::{Lua, Table, Value};
use protobuf::reflect::{
    EnumDescriptor, FieldDescriptor, FileDescriptor, MessageDescriptor, RuntimeFieldType,
    RuntimeType, ServiceDescriptor,
};

pub fn file_to_table(lua: &Lua, descriptor: &FileDescriptor) -> mlua::Result<Table> {
    let table = lua.create_table()?;
    table.set("kind", "file")?;
    table.set("name", descriptor.name())?;
    table.set("package", descriptor.package())?;
    table.set(
        "syntax",
        match descriptor.syntax() {
            protobuf::reflect::Syntax::Proto2 => "proto2",
            protobuf::reflect::Syntax::Proto3 => "proto3",
        },
    )?;

    let messages = lua.create_table()?;
    for message in descriptor.messages() {
        messages.push(message_summary_to_table(lua, &message)?)?;
    }
    table.set("messages", messages)?;

    let enums = lua.create_table()?;
    for enum_descriptor in descriptor.enums() {
        enums.push(enum_summary_to_table(lua, &enum_descriptor)?)?;
    }
    table.set("enums", enums)?;

    let services = lua.create_table()?;
    for service in descriptor.services() {
        services.push(service_to_table(lua, &service)?)?;
    }
    table.set("services", services)?;

    Ok(table)
}

pub fn message_to_table(lua: &Lua, descriptor: &MessageDescriptor) -> mlua::Result<Table> {
    let table = message_summary_to_table(lua, descriptor)?;
    table.set("package", descriptor.name_to_package())?;
    table.set("map_entry", descriptor.is_map_entry())?;

    let fields = lua.create_table()?;
    for field in descriptor.fields() {
        fields.push(field_to_table(lua, &field)?)?;
    }
    table.set("fields", fields)?;

    let oneofs = lua.create_table()?;
    for oneof in descriptor.oneofs() {
        let oneof_table = lua.create_table()?;
        oneof_table.set("name", oneof.name())?;
        oneof_table.set("full_name", oneof.full_name())?;
        oneof_table.set("synthetic", oneof.is_synthetic())?;
        let fields = lua.create_table()?;
        for field in oneof.fields() {
            fields.push(field.name().to_string())?;
        }
        oneof_table.set("fields", fields)?;
        oneofs.push(oneof_table)?;
    }
    table.set("oneofs", oneofs)?;

    let nested_messages = lua.create_table()?;
    for message in descriptor.nested_messages() {
        nested_messages.push(message_summary_to_table(lua, &message)?)?;
    }
    table.set("nested_messages", nested_messages)?;

    let nested_enums = lua.create_table()?;
    for enum_descriptor in descriptor.nested_enums() {
        nested_enums.push(enum_summary_to_table(lua, &enum_descriptor)?)?;
    }
    table.set("nested_enums", nested_enums)?;

    Ok(table)
}

pub fn enum_to_table(lua: &Lua, descriptor: &EnumDescriptor) -> mlua::Result<Table> {
    let table = enum_summary_to_table(lua, descriptor)?;
    let values = lua.create_table()?;
    for value in descriptor.values() {
        let value_table = lua.create_table()?;
        value_table.set("name", value.name())?;
        value_table.set("full_name", value.full_name())?;
        value_table.set("number", value.value())?;
        values.push(value_table)?;
    }
    table.set("values", values)?;
    Ok(table)
}

pub fn service_to_table(lua: &Lua, descriptor: &ServiceDescriptor) -> mlua::Result<Table> {
    let table = lua.create_table()?;
    table.set("kind", "service")?;
    table.set("name", descriptor.proto().name())?;

    let methods = lua.create_table()?;
    for method in descriptor.methods() {
        let method_table = lua.create_table()?;
        method_table.set("name", method.proto().name())?;
        method_table.set("input_type", method.input_type().full_name().to_string())?;
        method_table.set("output_type", method.output_type().full_name().to_string())?;
        method_table.set("client_streaming", method.proto().client_streaming())?;
        method_table.set("server_streaming", method.proto().server_streaming())?;
        methods.push(method_table)?;
    }
    table.set("methods", methods)?;

    Ok(table)
}

fn message_summary_to_table(lua: &Lua, descriptor: &MessageDescriptor) -> mlua::Result<Table> {
    let table = lua.create_table()?;
    table.set("kind", "message")?;
    table.set("name", descriptor.name())?;
    table.set("full_name", descriptor.full_name().to_string())?;
    Ok(table)
}

fn enum_summary_to_table(lua: &Lua, descriptor: &EnumDescriptor) -> mlua::Result<Table> {
    let table = lua.create_table()?;
    table.set("kind", "enum")?;
    table.set("name", descriptor.name())?;
    table.set("full_name", descriptor.full_name().to_string())?;
    Ok(table)
}

fn field_to_table(lua: &Lua, descriptor: &FieldDescriptor) -> mlua::Result<Table> {
    let table = lua.create_table()?;
    table.set("kind", "field")?;
    table.set("name", descriptor.name())?;
    table.set("json_name", descriptor.json_name())?;
    table.set("number", descriptor.number())?;
    table.set("full_name", descriptor.full_name())?;
    table.set("required", descriptor.is_required())?;
    table.set("repeated", descriptor.is_repeated())?;
    table.set("map", descriptor.is_map())?;
    table.set(
        "optional",
        descriptor.is_singular() && !descriptor.is_required(),
    )?;
    table.set("proto3_optional", descriptor.proto().proto3_optional())?;

    match descriptor.containing_oneof() {
        Some(oneof) => table.set("oneof", Value::String(lua.create_string(oneof.name())?))?,
        None => table.set("oneof", Value::Nil)?,
    }

    if descriptor.proto().has_default_value() {
        table.set("default", descriptor.proto().default_value())?;
    } else {
        table.set("default", Value::Nil)?;
    }

    match descriptor.runtime_field_type() {
        RuntimeFieldType::Singular(ty) => {
            table.set("cardinality", "singular")?;
            set_runtime_type(lua, &table, "type", ty)?;
        }
        RuntimeFieldType::Repeated(ty) => {
            table.set("cardinality", "repeated")?;
            set_runtime_type(lua, &table, "type", ty)?;
        }
        RuntimeFieldType::Map(key, value) => {
            table.set("cardinality", "map")?;
            set_runtime_type(lua, &table, "key_type", key)?;
            set_runtime_type(lua, &table, "value_type", value)?;
        }
    }

    Ok(table)
}

fn set_runtime_type(lua: &Lua, table: &Table, key: &str, ty: RuntimeType) -> mlua::Result<()> {
    match ty {
        RuntimeType::I32 => table.set(key, "int32"),
        RuntimeType::I64 => table.set(key, "int64"),
        RuntimeType::U32 => table.set(key, "uint32"),
        RuntimeType::U64 => table.set(key, "uint64"),
        RuntimeType::F32 => table.set(key, "float"),
        RuntimeType::F64 => table.set(key, "double"),
        RuntimeType::Bool => table.set(key, "bool"),
        RuntimeType::String => table.set(key, "string"),
        RuntimeType::VecU8 => table.set(key, "bytes"),
        RuntimeType::Enum(descriptor) => {
            table.set(key, "enum")?;
            table.set(format!("{key}_name"), descriptor.full_name().to_string())
        }
        RuntimeType::Message(descriptor) => {
            table.set(key, "message")?;
            table.set(format!("{key}_name"), descriptor.full_name().to_string())?;
            if descriptor.is_map_entry() {
                table.set(format!("{key}_map_entry"), true)?;
            }
            let nested = lua.create_table()?;
            nested.set("name", descriptor.name())?;
            nested.set("full_name", descriptor.full_name().to_string())?;
            table.set(format!("{key}_message"), nested)
        }
    }
}
