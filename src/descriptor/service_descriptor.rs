use std::ops::{Deref, DerefMut};
use mlua::prelude::LuaUserData;
use mlua::UserDataMethods;
use protobuf::reflect::ServiceDescriptor;
use crate::descriptor::method_descriptor::LuaMethodDescriptor;
use crate::descriptor_proto::service_descriptor_proto::LuaServiceDescriptorProto;

#[derive(Clone, Eq, PartialEq)]
pub struct LuaServiceDescriptor(ServiceDescriptor);

impl Deref for LuaServiceDescriptor {
    type Target = ServiceDescriptor;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaServiceDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<ServiceDescriptor> for LuaServiceDescriptor {
    fn from(value: ServiceDescriptor) -> Self {
        LuaServiceDescriptor(value)
    }
}

impl LuaUserData for LuaServiceDescriptor {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("proto", |_, this, ()| {
            let proto: LuaServiceDescriptorProto = this.proto().clone().into();
            Ok(proto)
        });
        methods.add_method("methods", |_, this, ()| {
            let methods: Vec<LuaMethodDescriptor> = this.methods().map(From::from).collect();
            Ok(methods)
        });
    }
}