use std::ops::{Deref, DerefMut};
use mlua::prelude::LuaUserData;
use mlua::UserDataMethods;
use protobuf::descriptor::FieldDescriptorProto;
use protobuf::MessageDyn;
use crate::descriptor::message_descriptor::LuaMessageDescriptor;

#[derive(PartialEq, Clone, Default, Debug)]
pub struct LuaFieldDescriptorProto(FieldDescriptorProto);

impl Deref for LuaFieldDescriptorProto {
    type Target = FieldDescriptorProto;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaFieldDescriptorProto {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<FieldDescriptorProto> for LuaFieldDescriptorProto {
    fn from(value: FieldDescriptorProto) -> Self {
        LuaFieldDescriptorProto(value)
    }
}

impl LuaUserData for LuaFieldDescriptorProto {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("descriptor_dyn", |_, this, ()| {
            let descriptor: LuaMessageDescriptor = From::from(this.descriptor_dyn());
            Ok(descriptor)
        })
    }
}