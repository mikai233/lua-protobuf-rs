use std::ops::{Deref, DerefMut};

use mlua::prelude::LuaUserData;
use mlua::UserDataMethods;
use protobuf::descriptor::MethodDescriptorProto;
use protobuf::MessageDyn;

use crate::descriptor::message_descriptor::LuaMessageDescriptor;

#[derive(PartialEq, Clone, Default, Debug)]
pub struct LuaMethodDescriptorProto(MethodDescriptorProto);

impl Deref for LuaMethodDescriptorProto {
    type Target = MethodDescriptorProto;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaMethodDescriptorProto {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<MethodDescriptorProto> for LuaMethodDescriptorProto {
    fn from(value: MethodDescriptorProto) -> Self {
        LuaMethodDescriptorProto(value)
    }
}

impl LuaUserData for LuaMethodDescriptorProto {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("descriptor_dyn", |_, this, ()| {
            let descriptor: LuaMessageDescriptor = From::from(this.descriptor_dyn());
            Ok(descriptor)
        })
    }
}