use std::ops::Deref;
use crate::descriptor::enum_descriptor::LuaEnumDescriptor;
use crate::descriptor::message_descriptor::LuaMessageDescriptor;
use derive_more::{Deref, From, Into};
use mlua::prelude::LuaUserData;
use mlua::UserDataMethods;
use protobuf::reflect::RuntimeType;

#[derive(Debug, Clone, Eq, PartialEq, Deref, From, Into)]
pub struct LuaRuntimeType(pub RuntimeType);

impl LuaUserData for LuaRuntimeType {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("is_i32", |_, this, ()| {
            Ok(matches!(this.deref(), RuntimeType::I32))
        });

        methods.add_method("is_i64", |_, this, ()| {
            Ok(matches!(this.deref(), RuntimeType::I64))
        });

        methods.add_method("is_u32", |_, this, ()| {
            Ok(matches!(this.deref(), RuntimeType::U32))
        });

        methods.add_method("is_u64", |_, this, ()| {
            Ok(matches!(this.deref(), RuntimeType::U64))
        });

        methods.add_method("is_f32", |_, this, ()| {
            Ok(matches!(this.deref(), RuntimeType::F32))
        });

        methods.add_method("is_f64", |_, this, ()| {
            Ok(matches!(this.deref(), RuntimeType::F64))
        });

        methods.add_method("is_bool", |_, this, ()| {
            Ok(matches!(this.deref(), RuntimeType::Bool))
        });

        methods.add_method("is_string", |_, this, ()| {
            Ok(matches!(this.deref(), RuntimeType::String))
        });

        methods.add_method("is_vec_u8", |_, this, ()| {
            Ok(matches!(this.deref(), RuntimeType::VecU8))
        });

        methods.add_method("is_enum", |_, this, ()| {
            Ok(matches!(this.deref(), RuntimeType::Enum(_)))
        });

        methods.add_method("get_enum", |_, this, ()| {
            if let RuntimeType::Enum(descriptor) = this.deref() {
                let descriptor: LuaEnumDescriptor = From::from(descriptor.clone());
                Ok(Some(descriptor))
            } else {
                Ok(None)
            }
        });

        methods.add_method("is_message", |_, this, ()| {
            Ok(matches!(this.deref(), RuntimeType::Message(_)))
        });

        methods.add_method("get_message", |_, this, ()| {
            if let RuntimeType::Message(message) = this.deref() {
                let descriptor: LuaMessageDescriptor = From::from(message.clone());
                Ok(Some(descriptor))
            } else {
                Ok(None)
            }
        });
    }
}
