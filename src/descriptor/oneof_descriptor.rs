use std::ops::{Deref, DerefMut};
use mlua::prelude::LuaUserData;
use mlua::UserDataMethods;
use protobuf::reflect::OneofDescriptor;
use crate::descriptor::field_descriptor::LuaFieldDescriptor;
use crate::descriptor::message_descriptor::LuaMessageDescriptor;

pub struct LuaOneofDescriptor(OneofDescriptor);

impl Deref for LuaOneofDescriptor {
    type Target = OneofDescriptor;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaOneofDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<OneofDescriptor> for LuaOneofDescriptor {
    fn from(value: OneofDescriptor) -> Self {
        LuaOneofDescriptor(value)
    }
}

impl LuaUserData for LuaOneofDescriptor {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("name", |_, this, ()| {
            Ok(this.name().to_string())
        });
        methods.add_method("containing_message", |_, this, ()| {
            let descriptor: LuaMessageDescriptor = From::from(this.containing_message());
            Ok(descriptor)
        });
        methods.add_method("is_synthetic", |_, this, ()| {
            Ok(this.is_synthetic())
        });
        methods.add_method("full_name", |_, this, ()| {
            Ok(this.full_name())
        });
        methods.add_method("fields", |_, this, ()| {
            let descriptors: Vec<LuaFieldDescriptor> = this.fields().map(From::from).collect();
            Ok(descriptors)
        });
    }
}