use std::collections::HashSet;
use std::ops::Deref;

use anyhow::{anyhow, Context};
use mlua::prelude::LuaValue;
use mlua::{Integer, Lua, Number, Table, Value};
use protobuf::reflect::{MessageDescriptor, ReflectValueBox, ReflectValueRef, RuntimeFieldType, RuntimeType};
use protobuf::MessageDyn;

#[derive(Copy, Clone, Default)]
pub struct LuaProtoCodec;

impl LuaProtoCodec {
    pub fn encode_message(&self, lua_message: Table, descriptor: &MessageDescriptor) -> anyhow::Result<Box<dyn MessageDyn>> {
        let name = descriptor.name();
        let mut message = descriptor.new_instance();
        for pair in lua_message.pairs::<Value, Value>() {
            let (field_key, field_value) = pair?;
            let field_key = field_key.as_str().ok_or(anyhow!("message {} expect a string key",name))?;
            let field_descriptor = descriptor.field_by_name(&field_key).ok_or(anyhow!("message {} {} not found",name,field_key))?;
            match field_descriptor.runtime_field_type() {
                RuntimeFieldType::Singular(ty) => {
                    let boxed_value = self.box_value(name, &field_key, &ty, field_value)?;
                    field_descriptor.set_singular_field(message.as_mut(), boxed_value);
                }
                RuntimeFieldType::Repeated(ty) => {
                    let mut field_repeated = field_descriptor.mut_repeated(message.as_mut());
                    let table = field_value.as_table().ok_or(anyhow!("message {} {} expect a table",name,field_key))?.clone();
                    for v in table.sequence_values::<Value>() {
                        let v = v?;
                        let boxed_value = self.box_value(name, &field_key, &ty, v)?;
                        field_repeated.push(boxed_value);
                    }
                }
                RuntimeFieldType::Map(k_ty, v_ty) => {
                    let mut field_map = field_descriptor.mut_map(message.as_mut());
                    let table = field_value.as_table().ok_or(anyhow!("message {} {} expect a table",name,field_key))?.clone();
                    for pair in table.pairs::<Value, Value>() {
                        let (key, value) = pair?;
                        let key = self.box_value(name, &field_key, &k_ty, key)?;
                        let value = self.box_value(name, &field_key, &v_ty, value)?;
                        field_map.insert(key, value);
                    }
                }
            }
        }
        Ok(message)
    }

    pub fn decode_message(&self, message: &dyn MessageDyn, lua: &Lua) -> anyhow::Result<Table> {
        let lua_message = lua.create_table()?;
        let descriptor = message.descriptor_dyn();
        let message_name = descriptor.name();
        let mut oneof_field = HashSet::new();
        for oneof_descriptor in descriptor.oneofs() {
            for field in oneof_descriptor.fields() {
                oneof_field.insert(field.name().to_string());
            }
        }
        for field in descriptor.fields() {
            let field_name = field.name();
            match field.runtime_field_type() {
                RuntimeFieldType::Singular(_) => {
                    if oneof_field.contains(field_name) {
                        if let Some(value) = field.get_singular(message) {
                            let field_table = self.unbox_value(message_name, field_name, value, lua)?;
                            lua_message.set(field_name, field_table)?;
                        }
                    } else {
                        let value = field.get_singular_field_or_default(message);
                        let field_table = self.unbox_value(message_name, field_name, value, lua)?;
                        lua_message.set(field_name, field_table)?;
                    }
                }
                RuntimeFieldType::Repeated(_) => {
                    let field_table = lua.create_table()?;
                    let values = field.get_repeated(message);
                    for value in values {
                        let v = self.unbox_value(message_name, field_name, value, lua)?;
                        field_table.push(v)?;
                    }
                    lua_message.set(field_name, field_table)?;
                }
                RuntimeFieldType::Map(_, _) => {
                    let field_table = lua.create_table()?;
                    let maps = field.get_map(message);
                    for (k, v) in maps.into_iter() {
                        let k = self.unbox_value(message_name, field_name, k, lua)?;
                        let v = self.unbox_value(message_name, field_name, v, lua)?;
                        field_table.set(k, v)?;
                    }
                    lua_message.set(field_name, field_table)?;
                }
            }
        }
        Ok(lua_message)
    }

    pub fn box_value(&self, name: &str, field: &str, ty: &RuntimeType, value: Value) -> anyhow::Result<ReflectValueBox> {
        fn value_cast_error(message: &str, field: &str, value: &str, ty: &str) -> anyhow::Error {
            anyhow!("message {} {} {} cannot cast to {}",message,field,value,ty)
        }
        let value_ty = self.fmt_value(&value);
        let value_box = match ty {
            RuntimeType::I32 => {
                let value = value.as_i32().ok_or(value_cast_error(name, field, &value_ty, "i32"))?;
                ReflectValueBox::I32(value)
            }
            RuntimeType::I64 => {
                let value = value.as_i64().ok_or(value_cast_error(name, field, &value_ty, "i64"))?;
                ReflectValueBox::I64(value)
            }
            RuntimeType::U32 => {
                let value = value.as_u32().ok_or(value_cast_error(name, field, &value_ty, "u32"))?;
                ReflectValueBox::U32(value)
            }
            RuntimeType::U64 => {
                let value = value.as_u64().ok_or(value_cast_error(name, field, &value_ty, "u64"))?;
                ReflectValueBox::U64(value)
            }
            RuntimeType::F32 => {
                let value = value.as_f32().ok_or(value_cast_error(name, field, &value_ty, "f32"))?;
                ReflectValueBox::F32(value)
            }
            RuntimeType::F64 => {
                let value = value.as_f64().ok_or(value_cast_error(name, field, &value_ty, "f64"))?;
                ReflectValueBox::F64(value)
            }
            RuntimeType::Bool => {
                let value = value.as_boolean().ok_or(value_cast_error(name, field, &value_ty, "bool"))?;
                ReflectValueBox::Bool(value)
            }
            RuntimeType::String => {
                let value = value.as_string_lossy().ok_or(value_cast_error(name, field, &value_ty, "string"))?;
                ReflectValueBox::String(value.to_string())
            }
            RuntimeType::VecU8 => {
                let table = value.as_table().ok_or(value_cast_error(name, field, &value_ty, "table"))?;
                let len = table.len()?;
                let mut bytes = Vec::with_capacity(len as usize);
                for byte in table.clone().sequence_values::<u8>() {
                    let byte = anyhow::Context::context(byte, format!("message {} {} expect u8 table", name, field))?;
                    bytes.push(byte);
                }
                ReflectValueBox::Bytes(bytes)
            }
            RuntimeType::Enum(descriptor) => {
                let value = value.as_i32().ok_or(value_cast_error(name, field, &value_ty, "i32"))?;
                descriptor.value_by_number(value).ok_or(anyhow!("incorrect number of enum {}",descriptor.name()))?;
                ReflectValueBox::Enum(descriptor.clone(), value)
            }
            RuntimeType::Message(descriptor) => {
                let table = value.as_table().ok_or(value_cast_error(name, field, &value_ty, "i32"))?;
                let message = self.encode_message(table.clone(), descriptor)?;
                ReflectValueBox::Message(message)
            }
        };
        Ok(value_box)
    }

    pub fn unbox_value(&self, message_name: &str, field_name: &str, value: ReflectValueRef, lua: &Lua) -> anyhow::Result<LuaValue> {
        let lua_value = match value {
            ReflectValueRef::U32(u) => { Value::Integer(Integer::from(u)) }
            ReflectValueRef::U64(u) => {
                let u = u32::try_from(u).context(format!("message {} {} cannot cast u64 value {} to u32", message_name, field_name, u))?;
                Value::Integer(Integer::from(u))
            }
            ReflectValueRef::I32(i) => { Value::Integer(Integer::from(i)) }
            ReflectValueRef::I64(i) => { Value::Integer(Integer::from(i)) }
            ReflectValueRef::F32(f) => { Value::Number(Number::from(f)) }
            ReflectValueRef::F64(f) => { Value::Number(Number::from(f)) }
            ReflectValueRef::Bool(b) => { Value::Boolean(b) }
            ReflectValueRef::String(s) => {
                let lua_string = lua.create_string(s)?;
                Value::String(lua_string)
            }
            ReflectValueRef::Bytes(bytes) => {
                let table = lua.create_table()?;
                for byte in bytes {
                    table.push(*byte)?;
                }
                Value::Table(table)
            }
            ReflectValueRef::Enum(_, i) => {
                Value::Integer(Integer::from(i))
            }
            ReflectValueRef::Message(m) => {
                let table = self.decode_message(m.deref(), lua)?;
                Value::Table(table)
            }
        };
        Ok(lua_value)
    }

    fn fmt_value(&self, value: &Value) -> String {
        let name = match value {
            Value::Nil => { "Nil" }
            Value::Boolean(_) => { "Boolean" }
            Value::LightUserData(_) => { "LightUserData" }
            Value::Integer(_) => { "Integer" }
            Value::Number(_) => { "Number" }
            Value::String(_) => { "String" }
            Value::Table(_) => { "Table" }
            Value::Function(_) => { "Function" }
            Value::Thread(_) => { "Thread" }
            Value::UserData(_) => { "UserData" }
            Value::Error(_) => { "Error" }
            #[cfg(any(feature = "luau", doc))]
            Value::Vector(_) => { "Vector" }
            #[cfg(any(feature = "luau", doc))]
            Value::Buffer(_) => { "Buffer" }
            Value::Other(_) => { "Other" }
        };
        name.to_string()
    }
}