use crate::descriptor::method_descriptor::LuaMethodDescriptor;
use crate::descriptor_proto::service_descriptor_proto::LuaServiceDescriptorProto;
use derive_more::{Deref, From, Into};
use mlua::prelude::LuaUserData;
use mlua::UserDataMethods;
use protobuf::reflect::ServiceDescriptor;

#[derive(Clone, Eq, PartialEq, Deref, From, Into)]
pub struct LuaServiceDescriptor(pub ServiceDescriptor);

impl LuaUserData for LuaServiceDescriptor {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
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