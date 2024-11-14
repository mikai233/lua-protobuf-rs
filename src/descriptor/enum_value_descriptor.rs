use crate::descriptor::enum_descriptor::LuaEnumDescriptor;
use mlua::prelude::LuaUserData;
use mlua::UserDataMethods;
use protobuf::reflect::EnumValueDescriptor;
use std::ops::{Deref, DerefMut};

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
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("name", |_, this, ()| Ok(this.name().to_string()));
        methods.add_method("full_name", |_, this, ()| Ok(this.full_name().to_string()));
        methods.add_method("value", |_, this, ()| Ok(this.value()));
        methods.add_method("enum_descriptor", |_, this, ()| {
            let descriptor: LuaEnumDescriptor = this.enum_descriptor().clone().into();
            Ok(descriptor)
        });
    }
}
