use std::ops::{Deref, DerefMut};
use mlua::prelude::LuaUserData;
use mlua::UserDataMethods;
use protobuf::descriptor::OneofDescriptorProto;
use protobuf::MessageDyn;
use crate::descriptor::message_descriptor::LuaMessageDescriptor;

#[derive(PartialEq, Clone, Default, Debug)]
pub struct LuaOneofDescriptorProto(OneofDescriptorProto);

impl Deref for LuaOneofDescriptorProto {
    type Target = OneofDescriptorProto;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaOneofDescriptorProto {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<OneofDescriptorProto> for LuaOneofDescriptorProto {
    fn from(value: OneofDescriptorProto) -> Self {
        LuaOneofDescriptorProto(value)
    }
}

impl LuaUserData for LuaOneofDescriptorProto {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("descriptor_dyn", |_, this, ()| {
            let descriptor: LuaMessageDescriptor = From::from(this.descriptor_dyn());
            Ok(descriptor)
        })
    }
}