use mlua::prelude::LuaUserData;
use mlua::UserDataMethods;
use protobuf::descriptor::FileDescriptorProto;
use protobuf::MessageDyn;

use crate::descriptor::message_descriptor::LuaMessageDescriptor;

#[derive(
    PartialEq,
    Clone,
    Default,
    Debug,
    mlua::FromLua,
    derive_more::Deref,
    derive_more::DerefMut,
    derive_more::From,
    derive_more::Into,
)]
pub struct LuaFileDescriptorProto(FileDescriptorProto);

impl LuaUserData for LuaFileDescriptorProto {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("descriptor_dyn", |_, this, ()| {
            let descriptor: LuaMessageDescriptor = From::from(this.descriptor_dyn());
            Ok(descriptor)
        })
    }
}
