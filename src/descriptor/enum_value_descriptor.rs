use std::ops::{Deref, DerefMut};
use mlua::prelude::LuaUserData;
use mlua::UserDataMethods;
use protobuf::reflect::EnumValueDescriptor;
use crate::descriptor::enum_descriptor::LuaEnumDescriptor;

pub struct LuaEnumValueDescriptor(EnumValueDescriptor);

impl Deref for LuaEnumValueDescriptor {
    type Target = EnumValueDescriptor;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaEnumValueDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<EnumValueDescriptor> for LuaEnumValueDescriptor {
    fn from(value: EnumValueDescriptor) -> Self {
        LuaEnumValueDescriptor(value)
    }
}

impl LuaUserData for LuaEnumValueDescriptor {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("name", |_, this, ()| {
            Ok(this.name().to_string())
        });
        methods.add_method("full_name", |_, this, ()| {
            Ok(this.full_name().to_string())
        });
        methods.add_method("value", |_, this, ()| {
            Ok(this.value())
        });
        methods.add_method("enum_descriptor", |_, this, ()| {
            let descriptor: LuaEnumDescriptor = From::from(this.enum_descriptor().clone());
            Ok(descriptor)
        });
    }
}