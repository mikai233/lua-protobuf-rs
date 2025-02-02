use crate::descriptor::field_descriptor::LuaFieldDescriptor;
use crate::descriptor::message_descriptor::LuaMessageDescriptor;
use crate::descriptor_proto::oneof_descriptor_proto::LuaOneofDescriptorProto;
use derive_more::{Deref, From, Into};
use mlua::prelude::LuaUserData;
use mlua::UserDataMethods;
use protobuf::reflect::OneofDescriptor;

#[derive(Eq, PartialEq, Clone, Debug, Deref, From, Into)]
pub struct LuaOneofDescriptor(pub OneofDescriptor);

impl LuaUserData for LuaOneofDescriptor {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("proto", |_, this, ()| {
            let proto: LuaOneofDescriptorProto = this.proto().clone().into();
            Ok(proto)
        });

        methods.add_method("name", |_, this, ()| {
            Ok(this.name().to_string())
        });
        
        methods.add_method("containing_message", |_, this, ()| {
            let descriptor: LuaMessageDescriptor = this.containing_message().into();
            Ok(descriptor)
        });

        methods.add_method("is_synthetic", |_, this, ()| {
            Ok(this.is_synthetic())
        });

        methods.add_method("full_name", |_, this, ()| {
            Ok(this.full_name())
        });
        
        methods.add_method("fields", |_, this, ()| {
            let descriptors: Vec<LuaFieldDescriptor> = this.fields().map(From::from).collect();
            Ok(descriptors)
        });
    }
}