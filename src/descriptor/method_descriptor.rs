use std::ops::{Deref, DerefMut};
use mlua::prelude::LuaUserData;
use mlua::UserDataMethods;
use protobuf::reflect::MethodDescriptor;
use crate::descriptor::message_descriptor::LuaMessageDescriptor;
use crate::descriptor_proto::method_descriptor_proto::LuaMethodDescriptorProto;

pub struct LuaMethodDescriptor(MethodDescriptor);

impl Deref for LuaMethodDescriptor {
    type Target = MethodDescriptor;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaMethodDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<MethodDescriptor> for LuaMethodDescriptor {
    fn from(value: MethodDescriptor) -> Self {
        LuaMethodDescriptor(value)
    }
}

impl LuaUserData for LuaMethodDescriptor {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("proto", |_, this, ()| {
            let proto: LuaMethodDescriptorProto = this.proto().clone().into();
            Ok(proto)
        });
        methods.add_method("input_type", |_, this, ()| {
            let descriptor: LuaMessageDescriptor = this.input_type().into();
            Ok(descriptor)
        });
        methods.add_method("input_type", |_, this, ()| {
            let descriptor: LuaMessageDescriptor = this.output_type().into();
            Ok(descriptor)
        });
    }
}