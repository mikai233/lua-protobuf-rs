use std::ops::{Deref, DerefMut};
use mlua::prelude::LuaUserData;
use mlua::UserDataMethods;
use protobuf::descriptor::DescriptorProto;
use protobuf::MessageDyn;
use crate::descriptor::message_descriptor::LuaMessageDescriptor;

pub struct LuaDescriptorProto(DescriptorProto);

impl Deref for LuaDescriptorProto {
    type Target = DescriptorProto;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaDescriptorProto {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<DescriptorProto> for LuaDescriptorProto {
    fn from(value: DescriptorProto) -> Self {
        LuaDescriptorProto(value)
    }
}

impl LuaUserData for LuaDescriptorProto {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("descriptor_dyn", |_, this, ()| {
            let descriptor: LuaMessageDescriptor = From::from(this.descriptor_dyn());
            Ok(descriptor)
        })
    }
}