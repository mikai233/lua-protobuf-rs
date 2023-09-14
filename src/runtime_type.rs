use std::ops::{Deref, DerefMut};
use mlua::prelude::LuaUserData;
use mlua::UserDataFields;
use protobuf::reflect::RuntimeType;
use crate::descriptor::{LuaEnumDescriptor, LuaMessageDescriptor};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LuaRuntimeType(RuntimeType);

impl Deref for LuaRuntimeType {
    type Target = RuntimeType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaRuntimeType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<RuntimeType> for LuaRuntimeType {
    fn from(value: RuntimeType) -> Self {
        LuaRuntimeType(value)
    }
}

impl LuaUserData for LuaRuntimeType {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("i32", |_, this| {
            if let RuntimeType::I32 = this.deref() {
                Ok(Some("i32".to_string()))
            } else {
                Ok(None)
            }
        });
        fields.add_field_method_get("i64", |_, this| {
            if let RuntimeType::I64 = this.deref() {
                Ok(Some("i64".to_string()))
            } else {
                Ok(None)
            }
        });
        fields.add_field_method_get("u32", |_, this| {
            if let RuntimeType::U32 = this.deref() {
                Ok(Some("u32".to_string()))
            } else {
                Ok(None)
            }
        });
        fields.add_field_method_get("u64", |_, this| {
            if let RuntimeType::U64 = this.deref() {
                Ok(Some("u64".to_string()))
            } else {
                Ok(None)
            }
        });
        fields.add_field_method_get("f32", |_, this| {
            if let RuntimeType::F32 = this.deref() {
                Ok(Some("f32".to_string()))
            } else {
                Ok(None)
            }
        });
        fields.add_field_method_get("f64", |_, this| {
            if let RuntimeType::F64 = this.deref() {
                Ok(Some("f64".to_string()))
            } else {
                Ok(None)
            }
        });
        fields.add_field_method_get("bool", |_, this| {
            if let RuntimeType::Bool = this.deref() {
                Ok(Some("bool".to_string()))
            } else {
                Ok(None)
            }
        });
        fields.add_field_method_get("string", |_, this| {
            if let RuntimeType::String = this.deref() {
                Ok(Some("string".to_string()))
            } else {
                Ok(None)
            }
        });
        fields.add_field_method_get("bytes", |_, this| {
            if let RuntimeType::VecU8 = this.deref() {
                Ok(Some("bytes".to_string()))
            } else {
                Ok(None)
            }
        });
        fields.add_field_method_get("enum", |_, this| {
            if let RuntimeType::Enum(descriptor) = this.deref() {
                let descriptor: LuaEnumDescriptor = From::from(descriptor.clone());
                Ok(Some(descriptor))
            } else {
                Ok(None)
            }
        });
        fields.add_field_method_get("message", |_, this| {
            if let RuntimeType::Message(message) = this.deref() {
                let descriptor: LuaMessageDescriptor = From::from(message.clone());
                Ok(Some(descriptor))
            } else {
                Ok(None)
            }
        });
    }
}

