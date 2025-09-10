use crate::descriptor::message_descriptor::LuaMessageDescriptor;
use crate::descriptor_proto::method_descriptor_proto::LuaMethodDescriptorProto;
use derive_more::{Deref, From, Into};
use mlua::UserDataMethods;
use mlua::prelude::LuaUserData;
use protobuf::reflect::MethodDescriptor;

#[derive(Deref, From, Into)]
pub struct LuaMethodDescriptor(pub MethodDescriptor);

impl LuaUserData for LuaMethodDescriptor {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
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
