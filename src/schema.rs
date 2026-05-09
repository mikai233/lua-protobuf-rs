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
    table.set(
        "dependency",
        string_array(lua, descriptor.proto().dependency.iter())?,
    )?;
    table.set(
        "public_dependency",
        int_array(lua, descriptor.proto().public_dependency.iter().copied())?,
    )?;

    let messages = lua.create_table()?;
    let messages_by_name = lua.create_table()?;
    for message in descriptor.messages() {
        let message_table = message_summary_to_table(lua, &message)?;
        messages_by_name.set(message.full_name(), message_table.clone())?;
        messages.push(message_table)?;
    }
    table.set("messages", messages)?;
    table.set("messages_by_name", messages_by_name)?;

    let enums = lua.create_table()?;
    let enums_by_name = lua.create_table()?;
    for enum_descriptor in descriptor.enums() {
        let enum_table = enum_summary_to_table(lua, &enum_descriptor)?;
        enums_by_name.set(enum_descriptor.full_name(), enum_table.clone())?;
        enums.push(enum_table)?;
    }
    table.set("enums", enums)?;
    table.set("enums_by_name", enums_by_name)?;

    let services = lua.create_table()?;
    let services_by_name = lua.create_table()?;
    for service in descriptor.services() {
        let full_name = service_full_name(descriptor.package(), service.proto().name());
        let service_table = service_to_table_with_full_name(lua, &service, &full_name)?;
        services_by_name.set(service.proto().name(), service_table.clone())?;
        services_by_name.set(full_name, service_table.clone())?;
        services.push(service_table)?;
    }
    table.set("services", services)?;
    table.set("services_by_name", services_by_name)?;

    Ok(table)
}

pub fn message_to_table(lua: &Lua, descriptor: &MessageDescriptor) -> mlua::Result<Table> {
    let table = message_summary_to_table(lua, descriptor)?;
    table.set("file", descriptor.file_descriptor().name())?;
    table.set("package", descriptor.file_descriptor().package())?;
    table.set("relative_name", descriptor.name_to_package())?;
    table.set("map_entry", descriptor.is_map_entry())?;

    let fields = lua.create_table()?;
    let fields_by_name = lua.create_table()?;
    let fields_by_json_name = lua.create_table()?;
    let fields_by_number = lua.create_table()?;
    for field in descriptor.fields() {
        let field_table = field_to_table(lua, &field)?;
        fields_by_name.set(field.name(), field_table.clone())?;
        fields_by_json_name.set(field.json_name(), field_table.clone())?;
        fields_by_number.set(field.number(), field_table.clone())?;
        fields.push(field_table)?;
    }
    table.set("fields", fields)?;
    table.set("fields_by_name", fields_by_name)?;
    table.set("fields_by_json_name", fields_by_json_name)?;
    table.set("fields_by_number", fields_by_number)?;

    let oneofs = lua.create_table()?;
    let oneofs_by_name = lua.create_table()?;
    for oneof in descriptor.oneofs() {
        let oneof_table = lua.create_table()?;
        oneof_table.set("name", oneof.name())?;
        oneof_table.set("full_name", oneof.full_name())?;
        oneof_table.set("synthetic", oneof.is_synthetic())?;
        oneof_table.set("containing_message", descriptor.full_name())?;
        let fields = lua.create_table()?;
        let field_names = lua.create_table()?;
        let fields_by_name = lua.create_table()?;
        for field in oneof.fields() {
            let field_table = field_to_table(lua, &field)?;
            field_names.push(field.name().to_string())?;
            fields_by_name.set(field.name(), field_table.clone())?;
            fields.push(field_table)?;
        }
        oneof_table.set("fields", fields)?;
        oneof_table.set("field_names", field_names)?;
        oneof_table.set("fields_by_name", fields_by_name)?;
        oneofs_by_name.set(oneof.name(), oneof_table.clone())?;
        oneofs.push(oneof_table)?;
    }
    table.set("oneofs", oneofs)?;
    table.set("oneofs_by_name", oneofs_by_name)?;

    let nested_messages = lua.create_table()?;
    let nested_messages_by_name = lua.create_table()?;
    for message in descriptor.nested_messages() {
        let message_table = message_summary_to_table(lua, &message)?;
        nested_messages_by_name.set(message.full_name(), message_table.clone())?;
        nested_messages.push(message_table)?;
    }
    table.set("nested_messages", nested_messages)?;
    table.set("nested_messages_by_name", nested_messages_by_name)?;

    let nested_enums = lua.create_table()?;
    let nested_enums_by_name = lua.create_table()?;
    for enum_descriptor in descriptor.nested_enums() {
        let enum_table = enum_summary_to_table(lua, &enum_descriptor)?;
        nested_enums_by_name.set(enum_descriptor.full_name(), enum_table.clone())?;
        nested_enums.push(enum_table)?;
    }
    table.set("nested_enums", nested_enums)?;
    table.set("nested_enums_by_name", nested_enums_by_name)?;

    Ok(table)
}

pub fn enum_to_table(lua: &Lua, descriptor: &EnumDescriptor) -> mlua::Result<Table> {
    let table = enum_summary_to_table(lua, descriptor)?;
    table.set("relative_name", descriptor.name_to_package())?;
    let values = lua.create_table()?;
    let values_by_name = lua.create_table()?;
    let values_by_number = lua.create_table()?;
    for value in descriptor.values() {
        let value_table = enum_value_to_table(lua, value.name(), value.full_name(), value.value())?;
        values_by_name.set(value.name(), value_table.clone())?;
        values_by_number.set(value.value(), value_table.clone())?;
        values.push(value_table)?;
    }
    table.set("values", values)?;
    table.set("values_by_name", values_by_name)?;
    table.set("values_by_number", values_by_number)?;
    let default = descriptor.default_value();
    table.set(
        "default_value",
        enum_value_to_table(lua, default.name(), default.full_name(), default.value())?,
    )?;
    Ok(table)
}

pub fn service_to_table_with_full_name(
    lua: &Lua,
    descriptor: &ServiceDescriptor,
    full_name: &str,
) -> mlua::Result<Table> {
    let table = lua.create_table()?;
    table.set("kind", "service")?;
    table.set("name", descriptor.proto().name())?;
    table.set("full_name", full_name)?;

    let methods = lua.create_table()?;
    let methods_by_name = lua.create_table()?;
    for method in descriptor.methods() {
        let method_table = lua.create_table()?;
        method_table.set("name", method.proto().name())?;
        method_table.set("input_type", method.input_type().full_name().to_string())?;
        method_table.set(
            "input",
            message_summary_to_table(lua, &method.input_type())?,
        )?;
        method_table.set("output_type", method.output_type().full_name().to_string())?;
        method_table.set(
            "output",
            message_summary_to_table(lua, &method.output_type())?,
        )?;
        method_table.set("client_streaming", method.proto().client_streaming())?;
        method_table.set("server_streaming", method.proto().server_streaming())?;
        methods_by_name.set(method.proto().name(), method_table.clone())?;
        methods.push(method_table)?;
    }
    table.set("methods", methods)?;
    table.set("methods_by_name", methods_by_name)?;

    Ok(table)
}

fn service_full_name(package: &str, name: &str) -> String {
    if package.is_empty() {
        name.to_string()
    } else {
        format!("{package}.{name}")
    }
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
    table.set(
        "containing_message",
        descriptor.containing_message().full_name(),
    )?;
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
            let resolved = runtime_type_to_table(lua, &ty)?;
            set_runtime_type(lua, &table, "type", ty)?;
            table.set("resolved_type", resolved)?;
        }
        RuntimeFieldType::Repeated(ty) => {
            table.set("cardinality", "repeated")?;
            let item_type = runtime_type_to_table(lua, &ty)?;
            set_runtime_type(lua, &table, "type", ty)?;
            let resolved = lua.create_table()?;
            resolved.set("kind", "repeated")?;
            resolved.set("item", item_type)?;
            table.set("resolved_type", resolved)?;
        }
        RuntimeFieldType::Map(key, value) => {
            table.set("cardinality", "map")?;
            let key_type = runtime_type_to_table(lua, &key)?;
            let value_type = runtime_type_to_table(lua, &value)?;
            set_runtime_type(lua, &table, "key_type", key)?;
            set_runtime_type(lua, &table, "value_type", value)?;
            let resolved = lua.create_table()?;
            resolved.set("kind", "map")?;
            resolved.set("key", key_type)?;
            resolved.set("value", value_type)?;
            table.set("resolved_type", resolved)?;
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

fn runtime_type_to_table(lua: &Lua, ty: &RuntimeType) -> mlua::Result<Table> {
    let table = lua.create_table()?;
    match ty {
        RuntimeType::I32 => table.set("kind", "int32")?,
        RuntimeType::I64 => table.set("kind", "int64")?,
        RuntimeType::U32 => table.set("kind", "uint32")?,
        RuntimeType::U64 => table.set("kind", "uint64")?,
        RuntimeType::F32 => table.set("kind", "float")?,
        RuntimeType::F64 => table.set("kind", "double")?,
        RuntimeType::Bool => table.set("kind", "bool")?,
        RuntimeType::String => table.set("kind", "string")?,
        RuntimeType::VecU8 => table.set("kind", "bytes")?,
        RuntimeType::Enum(descriptor) => {
            table.set("kind", "enum")?;
            table.set("name", descriptor.name())?;
            table.set("full_name", descriptor.full_name().to_string())?;
            table.set("relative_name", descriptor.name_to_package())?;
        }
        RuntimeType::Message(descriptor) => {
            table.set("kind", "message")?;
            table.set("name", descriptor.name())?;
            table.set("full_name", descriptor.full_name().to_string())?;
            table.set("relative_name", descriptor.name_to_package())?;
            table.set("map_entry", descriptor.is_map_entry())?;
        }
    }
    Ok(table)
}

fn enum_value_to_table(
    lua: &Lua,
    name: &str,
    full_name: String,
    number: i32,
) -> mlua::Result<Table> {
    let table = lua.create_table()?;
    table.set("name", name)?;
    table.set("full_name", full_name)?;
    table.set("number", number)?;
    Ok(table)
}

fn string_array<'a>(
    lua: &Lua,
    values: impl IntoIterator<Item = &'a String>,
) -> mlua::Result<Table> {
    let table = lua.create_table()?;
    for value in values {
        table.push(value.as_str())?;
    }
    Ok(table)
}

fn int_array(lua: &Lua, values: impl IntoIterator<Item = i32>) -> mlua::Result<Table> {
    let table = lua.create_table()?;
    for value in values {
        table.push(value)?;
    }
    Ok(table)
}
