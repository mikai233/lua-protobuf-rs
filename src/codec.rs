use std::collections::HashSet;
use std::ops::Deref;

use anyhow::{Context, anyhow};
use mlua::prelude::LuaValue;
use mlua::{Integer, Lua, Number, Table, Value};
use protobuf::MessageDyn;
use protobuf::reflect::{
    MessageDescriptor, ReflectValueBox, ReflectValueRef, RuntimeFieldType, RuntimeType,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Int64Mode {
    String,
    Integer,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BytesMode {
    String,
    Table,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum EnumMode {
    Name,
    Number,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UnknownFieldMode {
    Error,
    Ignore,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CodecOptions {
    pub defaults: bool,
    pub int64: Int64Mode,
    pub bytes: BytesMode,
    pub enum_mode: EnumMode,
    pub unknown_fields: UnknownFieldMode,
}

impl Default for CodecOptions {
    fn default() -> Self {
        Self {
            defaults: false,
            int64: Int64Mode::String,
            bytes: BytesMode::String,
            enum_mode: EnumMode::Name,
            unknown_fields: UnknownFieldMode::Error,
        }
    }
}

impl CodecOptions {
    pub fn from_lua_table(table: Option<Table>) -> anyhow::Result<Self> {
        let Some(table) = table else {
            return Ok(Self::default());
        };

        let mut options = Self::default();
        if let Some(defaults) = table.get::<Option<bool>>("defaults")? {
            options.defaults = defaults;
        }
        if let Some(int64) = table.get::<Option<String>>("int64")? {
            options.int64 = match int64.as_str() {
                "string" => Int64Mode::String,
                "integer" => Int64Mode::Integer,
                _ => return Err(anyhow!("unknown int64 mode: {int64}")),
            };
        }
        if let Some(bytes) = table.get::<Option<String>>("bytes")? {
            options.bytes = match bytes.as_str() {
                "string" => BytesMode::String,
                "table" => BytesMode::Table,
                _ => return Err(anyhow!("unknown bytes mode: {bytes}")),
            };
        }
        if let Some(enum_mode) = table.get::<Option<String>>("enum")? {
            options.enum_mode = match enum_mode.as_str() {
                "name" => EnumMode::Name,
                "number" => EnumMode::Number,
                _ => return Err(anyhow!("unknown enum mode: {enum_mode}")),
            };
        }
        if let Some(unknown) = table.get::<Option<String>>("unknown")? {
            options.unknown_fields = match unknown.as_str() {
                "error" => UnknownFieldMode::Error,
                "ignore" => UnknownFieldMode::Ignore,
                _ => return Err(anyhow!("unknown unknown-field mode: {unknown}")),
            };
        }
        Ok(options)
    }
}

#[derive(Copy, Clone, Default)]
pub struct LuaProtoCodec;

impl LuaProtoCodec {
    pub fn encode_message(
        &self,
        lua_message: &Table,
        descriptor: &MessageDescriptor,
        options: CodecOptions,
    ) -> anyhow::Result<Box<dyn MessageDyn>> {
        self.encode_message_at(lua_message, descriptor, options, descriptor.full_name())
    }

    fn encode_message_at(
        &self,
        lua_message: &Table,
        descriptor: &MessageDescriptor,
        options: CodecOptions,
        path: &str,
    ) -> anyhow::Result<Box<dyn MessageDyn>> {
        let name = descriptor.full_name();
        let mut message = descriptor.new_instance();
        for pair in lua_message.pairs::<Value, Value>() {
            let (field_key, field_value) = pair?;
            let field_key = field_key
                .as_string()
                .ok_or(anyhow!("{} expects string field keys", path))?
                .to_str()?
                .to_string();
            let field_path = format!("{path}.{field_key}");
            let Some(field_descriptor) = descriptor.field_by_name_or_json_name(&field_key) else {
                match options.unknown_fields {
                    UnknownFieldMode::Error => {
                        return Err(anyhow!(
                            "{}: field not found in message {}",
                            field_path,
                            name
                        ));
                    }
                    UnknownFieldMode::Ignore => continue,
                }
            };
            if field_value.is_nil() {
                field_descriptor.clear_field(message.as_mut());
                continue;
            }
            match field_descriptor.runtime_field_type() {
                RuntimeFieldType::Singular(ty) => {
                    let boxed_value = self.box_value_at(&field_path, &ty, field_value, options)?;
                    field_descriptor.set_singular_field(message.as_mut(), boxed_value);
                }
                RuntimeFieldType::Repeated(ty) => {
                    let mut field_repeated = field_descriptor.mut_repeated(message.as_mut());
                    let table = field_value
                        .as_table()
                        .ok_or(anyhow!("{} expects a table", field_path,))?;
                    for (index, v) in table.sequence_values::<Value>().enumerate() {
                        let value_path = format!("{}[{}]", field_path, index + 1);
                        let v = v.context(format!("{value_path}: invalid sequence value"))?;
                        let boxed_value = self.box_value_at(&value_path, &ty, v, options)?;
                        field_repeated.push(boxed_value);
                    }
                }
                RuntimeFieldType::Map(k_ty, v_ty) => {
                    let mut field_map = field_descriptor.mut_map(message.as_mut());
                    let table = field_value
                        .as_table()
                        .ok_or(anyhow!("{} expects a table", field_path,))?;
                    for pair in table.pairs::<Value, Value>() {
                        let (key, value) = pair?;
                        let key_label = Self::lua_key_label(&key);
                        let key_path = format!("{field_path}[{key_label}]");
                        let value_path = key_path.clone();
                        let key = self.box_value_at(&key_path, &k_ty, key, options)?;
                        let value = self.box_value_at(&value_path, &v_ty, value, options)?;
                        field_map.insert(key, value);
                    }
                }
            }
        }
        Ok(message)
    }

    pub fn decode_message(
        &self,
        lua: &Lua,
        message: &dyn MessageDyn,
        options: CodecOptions,
    ) -> anyhow::Result<Table> {
        let lua_message = lua.create_table()?;
        let descriptor = message.descriptor_dyn();
        let message_name = descriptor.full_name();
        let oneof_fields = Self::oneof_field_names(&descriptor);

        for field in descriptor.fields() {
            let field_name = field.name();
            match field.runtime_field_type() {
                RuntimeFieldType::Singular(_) => {
                    let value = if oneof_fields.contains(field_name) || !options.defaults {
                        field.get_singular(message)
                    } else {
                        Some(field.get_singular_field_or_default(message))
                    };
                    if let Some(value) = value {
                        let field_table =
                            self.unbox_value(message_name, field_name, value, lua, options)?;
                        lua_message.set(field_name, field_table)?;
                    }
                }
                RuntimeFieldType::Repeated(_) => {
                    if !options.defaults && !field.has_field(message) {
                        continue;
                    }
                    let field_table = lua.create_table()?;
                    let values = field.get_repeated(message);
                    for value in values {
                        let v = self.unbox_value(message_name, field_name, value, lua, options)?;
                        field_table.push(v)?;
                    }
                    lua_message.set(field_name, field_table)?;
                }
                RuntimeFieldType::Map(_, _) => {
                    if !options.defaults && !field.has_field(message) {
                        continue;
                    }
                    let field_table = lua.create_table()?;
                    let maps = field.get_map(message);
                    for (k, v) in &maps {
                        let k = self.unbox_value(message_name, field_name, k, lua, options)?;
                        let v = self.unbox_value(message_name, field_name, v, lua, options)?;
                        field_table.set(k, v)?;
                    }
                    lua_message.set(field_name, field_table)?;
                }
            }
        }
        Ok(lua_message)
    }

    pub fn box_value(
        &self,
        name: &str,
        field: &str,
        ty: &RuntimeType,
        value: Value,
        options: CodecOptions,
    ) -> anyhow::Result<ReflectValueBox> {
        self.box_value_at(&format!("{name}.{field}"), ty, value, options)
    }

    fn box_value_at(
        &self,
        path: &str,
        ty: &RuntimeType,
        value: Value,
        options: CodecOptions,
    ) -> anyhow::Result<ReflectValueBox> {
        fn value_cast_error(path: &str, value: &str, ty: &str) -> anyhow::Error {
            anyhow!("{}: value {} cannot be cast to {}", path, value, ty)
        }

        let value_ty = self.fmt_value(&value);
        let value_box = match ty {
            RuntimeType::I32 => {
                let value = value
                    .as_i32()
                    .ok_or(value_cast_error(path, value_ty, "int32"))?;
                ReflectValueBox::I32(value)
            }
            RuntimeType::I64 => {
                let value =
                    Self::lua_i64(value).ok_or(value_cast_error(path, value_ty, "int64"))?;
                ReflectValueBox::I64(value)
            }
            RuntimeType::U32 => {
                let value = value
                    .as_u32()
                    .ok_or(value_cast_error(path, value_ty, "uint32"))?;
                ReflectValueBox::U32(value)
            }
            RuntimeType::U64 => {
                let value =
                    Self::lua_u64(value).ok_or(value_cast_error(path, value_ty, "uint64"))?;
                ReflectValueBox::U64(value)
            }
            RuntimeType::F32 => {
                let value = value
                    .as_f32()
                    .ok_or(value_cast_error(path, value_ty, "float"))?;
                ReflectValueBox::F32(value)
            }
            RuntimeType::F64 => {
                let value = value
                    .as_f64()
                    .ok_or(value_cast_error(path, value_ty, "double"))?;
                ReflectValueBox::F64(value)
            }
            RuntimeType::Bool => {
                let value = value
                    .as_boolean()
                    .ok_or(value_cast_error(path, value_ty, "bool"))?;
                ReflectValueBox::Bool(value)
            }
            RuntimeType::String => {
                let value = value
                    .as_string()
                    .ok_or(value_cast_error(path, value_ty, "string"))?
                    .to_str()?
                    .to_string();
                ReflectValueBox::String(value)
            }
            RuntimeType::VecU8 => {
                let bytes = match options.bytes {
                    BytesMode::String => value
                        .as_string()
                        .ok_or(value_cast_error(path, value_ty, "binary string"))?
                        .as_bytes()
                        .to_vec(),
                    BytesMode::Table => {
                        let table = value.as_table().ok_or(value_cast_error(
                            path,
                            value_ty,
                            "byte table",
                        ))?;
                        let len = table.len()?;
                        let mut bytes = Vec::with_capacity(len as usize);
                        for (index, byte) in table.sequence_values::<u8>().enumerate() {
                            let byte = anyhow::Context::context(
                                byte,
                                format!("{}[{}]: expected u8", path, index + 1),
                            )?;
                            bytes.push(byte);
                        }
                        bytes
                    }
                };
                ReflectValueBox::Bytes(bytes)
            }
            RuntimeType::Enum(descriptor) => {
                let value = match value {
                    Value::String(s) => {
                        let name = s.to_str()?;
                        descriptor
                            .value_by_name(name.as_ref())
                            .ok_or(anyhow!(
                                "{}: unknown enum value {}.{}",
                                path,
                                descriptor.full_name(),
                                name
                            ))?
                            .value()
                    }
                    value => value.as_i32().ok_or(value_cast_error(
                        path,
                        value_ty,
                        "enum name or i32",
                    ))?,
                };
                descriptor.value_by_number(value).ok_or(anyhow!(
                    "{}: incorrect number of enum {}",
                    path,
                    descriptor.name()
                ))?;
                ReflectValueBox::Enum(descriptor.clone(), value)
            }
            RuntimeType::Message(descriptor) => {
                let table = value
                    .as_table()
                    .ok_or(value_cast_error(path, value_ty, "table"))?;
                let message = self.encode_message_at(table, descriptor, options, path)?;
                ReflectValueBox::Message(message)
            }
        };
        Ok(value_box)
    }

    pub fn unbox_value(
        &self,
        message_name: &str,
        field_name: &str,
        value: ReflectValueRef,
        lua: &Lua,
        options: CodecOptions,
    ) -> anyhow::Result<LuaValue> {
        let lua_value = match value {
            ReflectValueRef::U32(u) => Value::Integer(Integer::from(u)),
            ReflectValueRef::U64(u) => match options.int64 {
                Int64Mode::String => Value::String(lua.create_string(u.to_string())?),
                Int64Mode::Integer => {
                    let u = i64::try_from(u).context(format!(
                        "message {} field {} cannot cast u64 value {} to i64",
                        message_name, field_name, u
                    ))?;
                    Value::Integer(Integer::from(u))
                }
            },
            ReflectValueRef::I32(i) => Value::Integer(Integer::from(i)),
            ReflectValueRef::I64(i) => match options.int64 {
                Int64Mode::String => Value::String(lua.create_string(i.to_string())?),
                Int64Mode::Integer => Value::Integer(Integer::from(i)),
            },
            ReflectValueRef::F32(f) => Value::Number(Number::from(f)),
            ReflectValueRef::F64(f) => Value::Number(Number::from(f)),
            ReflectValueRef::Bool(b) => Value::Boolean(b),
            ReflectValueRef::String(s) => Value::String(lua.create_string(s)?),
            ReflectValueRef::Bytes(bytes) => match options.bytes {
                BytesMode::String => Value::String(lua.create_string(bytes)?),
                BytesMode::Table => {
                    let table = lua.create_table()?;
                    for byte in bytes {
                        table.push(*byte)?;
                    }
                    Value::Table(table)
                }
            },
            ReflectValueRef::Enum(descriptor, i) => match options.enum_mode {
                EnumMode::Name => match descriptor.value_by_number(i) {
                    Some(value) => Value::String(lua.create_string(value.name())?),
                    None => Value::Integer(Integer::from(i)),
                },
                EnumMode::Number => Value::Integer(Integer::from(i)),
            },
            ReflectValueRef::Message(m) => {
                let table = self.decode_message(lua, m.deref(), options)?;
                Value::Table(table)
            }
        };
        Ok(lua_value)
    }

    fn oneof_field_names(descriptor: &MessageDescriptor) -> HashSet<String> {
        let mut oneof_field = HashSet::new();
        for oneof_descriptor in descriptor.oneofs() {
            for field in oneof_descriptor.fields() {
                oneof_field.insert(field.name().to_string());
            }
        }
        oneof_field
    }

    fn lua_i64(value: Value) -> Option<i64> {
        match value {
            Value::Integer(i) => Some(i),
            Value::String(s) => s.to_str().ok()?.parse().ok(),
            _ => None,
        }
    }

    fn lua_u64(value: Value) -> Option<u64> {
        match value {
            Value::Integer(i) => u64::try_from(i).ok(),
            Value::String(s) => s.to_str().ok()?.parse().ok(),
            _ => None,
        }
    }

    fn fmt_value(&self, value: &Value) -> &'static str {
        match value {
            Value::Nil => "Nil",
            Value::Boolean(_) => "Boolean",
            Value::LightUserData(_) => "LightUserData",
            Value::Integer(_) => "Integer",
            Value::Number(_) => "Number",
            Value::String(_) => "String",
            Value::Table(_) => "Table",
            Value::Function(_) => "Function",
            Value::Thread(_) => "Thread",
            Value::UserData(_) => "UserData",
            Value::Error(_) => "Error",
            #[cfg(any(feature = "luau", doc))]
            Value::Vector(_) => "Vector",
            #[cfg(any(feature = "luau", doc))]
            Value::Buffer(_) => "Buffer",
            Value::Other(_) => "Other",
        }
    }

    fn lua_key_label(value: &Value) -> String {
        match value {
            Value::String(s) => match s.to_str() {
                Ok(s) => format!("{s:?}"),
                Err(_) => "\"<non-utf8>\"".to_string(),
            },
            Value::Integer(i) => i.to_string(),
            Value::Number(n) => n.to_string(),
            Value::Boolean(b) => b.to_string(),
            _ => format!("<{}>", LuaProtoCodec.fmt_value(value)),
        }
    }
}
