use std::path::Path;

use anyhow::Context;
use protobuf::reflect::{
    EnumDescriptor, FileDescriptor, MessageDescriptor, RuntimeFieldType, RuntimeType,
};

pub(crate) fn generate<'a>(
    files: impl IntoIterator<Item = &'a FileDescriptor>,
    output_dir: impl AsRef<Path>,
) -> anyhow::Result<()> {
    let output_dir = output_dir.as_ref();
    for file in files {
        let Some(name) = file.name().strip_suffix(".proto") else {
            continue;
        };
        let mut chunks = Vec::new();

        for message in file.messages() {
            write_message(&message, &mut chunks);
        }
        for enum_descriptor in file.enums() {
            chunks.push(enum_to_lua(&enum_descriptor));
        }

        let file_path = output_dir.join(format!("{name}.lua"));
        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent).context(format!(
                "failed create lua output directory {}",
                parent.to_string_lossy()
            ))?;
        }
        std::fs::write(&file_path, chunks.join("\n")).context(format!(
            "failed write lua to file {}",
            file_path.to_string_lossy()
        ))?;
    }
    Ok(())
}

fn write_message(descriptor: &MessageDescriptor, chunks: &mut Vec<String>) {
    if descriptor.is_map_entry() {
        return;
    }

    for nested_message in descriptor.nested_messages() {
        write_message(&nested_message, chunks);
    }
    for nested_enum in descriptor.nested_enums() {
        chunks.push(enum_to_lua(&nested_enum));
    }

    chunks.push(message_to_lua(descriptor));
}

fn message_to_lua(descriptor: &MessageDescriptor) -> String {
    let class_name = lua_class_name(descriptor.full_name());
    let mut lines = vec![format!("---@class {class_name}")];

    for field in descriptor.fields() {
        let optional = if field.is_required() { "" } else { "?" };
        lines.push(format!(
            "---@field {}{} {}",
            field.name(),
            optional,
            lua_field_type(field.runtime_field_type())
        ));
    }

    lines.push(format!("local {class_name} = {{ }}"));
    lines.push(String::new());
    lines.join("\n")
}

fn enum_to_lua(descriptor: &EnumDescriptor) -> String {
    let class_name = lua_class_name(descriptor.full_name());
    let mut lines = vec![format!("---@class {class_name}")];
    let mut values = Vec::new();
    for value in descriptor.values() {
        lines.push(format!(
            "---@field {} integer {}",
            value.name(),
            value.value()
        ));
        values.push(format!("{} = {}", value.name(), value.value()));
    }

    lines.push(format!("local {class_name} = {{ {} }}", values.join(", ")));
    lines.push(String::new());
    lines.join("\n")
}

fn lua_field_type(field_type: RuntimeFieldType) -> String {
    match field_type {
        RuntimeFieldType::Singular(ty) => lua_type(ty),
        RuntimeFieldType::Repeated(ty) => format!("{}[]", lua_type(ty)),
        RuntimeFieldType::Map(key, value) => {
            format!("table<{},{}>", lua_type(key), lua_type(value))
        }
    }
}

fn lua_type(ty: RuntimeType) -> String {
    match ty {
        RuntimeType::I32 | RuntimeType::U32 => "integer".to_string(),
        RuntimeType::I64 | RuntimeType::U64 => "string".to_string(),
        RuntimeType::F32 | RuntimeType::F64 => "number".to_string(),
        RuntimeType::Bool => "boolean".to_string(),
        RuntimeType::String | RuntimeType::VecU8 => "string".to_string(),
        RuntimeType::Enum(e) => lua_class_name(e.full_name()),
        RuntimeType::Message(m) => lua_class_name(m.full_name()),
    }
}

fn lua_class_name(full_name: &str) -> String {
    full_name.replace('.', "_")
}
