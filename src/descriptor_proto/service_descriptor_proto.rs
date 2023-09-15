use std::ops::{Deref, DerefMut};
use mlua::prelude::LuaUserData;
use mlua::UserDataMethods;
use protobuf::descriptor::ServiceDescriptorProto;
use protobuf::MessageDyn;
use crate::descriptor::message_descriptor::LuaMessageDescriptor;

#[derive(PartialEq, Clone, Default, Debug)]
pub struct LuaServiceDescriptorProto(ServiceDescriptorProto);

impl Deref for LuaServiceDescriptorProto {
    type Target = ServiceDescriptorProto;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaServiceDescriptorProto {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<ServiceDescriptorProto> for LuaServiceDescriptorProto {
    fn from(value: ServiceDescriptorProto) -> Self {
        LuaServiceDescriptorProto(value)
    }
}

impl LuaUserData for LuaServiceDescriptorProto {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("descriptor_dyn", |_, this, ()| {
            let descriptor: LuaMessageDescriptor = From::from(this.descriptor_dyn());
            Ok(descriptor)
        })
    }
}