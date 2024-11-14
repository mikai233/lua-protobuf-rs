use crate::descriptor::message_descriptor::LuaMessageDescriptor;
use mlua::prelude::LuaUserData;
use mlua::UserDataMethods;
use protobuf::descriptor::ServiceDescriptorProto;
use protobuf::MessageDyn;

#[derive(
    PartialEq,
    Clone,
    Default,
    Debug,
    derive_more::Deref,
    derive_more::DerefMut,
    derive_more::From,
    derive_more::Into,
)]
pub struct LuaServiceDescriptorProto(ServiceDescriptorProto);

impl LuaUserData for LuaServiceDescriptorProto {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("descriptor_dyn", |_, this, ()| {
            let descriptor: LuaMessageDescriptor = From::from(this.descriptor_dyn());
            Ok(descriptor)
        })
    }
}
