use std::ops::{Deref, DerefMut};

use mlua::UserDataMethods;
use mlua::prelude::LuaUserData;
use protobuf::descriptor::FileDescriptorProto;
use protobuf::MessageDyn;

use crate::descriptor::message_descriptor::LuaMessageDescriptor;

#[derive(PartialEq, Clone, Default, Debug, mlua::FromLua)]
pub struct LuaFileDescriptorProto(FileDescriptorProto);

impl Deref for LuaFileDescriptorProto {
    type Target = FileDescriptorProto;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaFileDescriptorProto {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<FileDescriptorProto> for LuaFileDescriptorProto {
    fn from(value: FileDescriptorProto) -> Self {
        LuaFileDescriptorProto(value)
    }
}

impl Into<FileDescriptorProto> for LuaFileDescriptorProto {
    fn into(self) -> FileDescriptorProto {
        self.0
    }
}

impl LuaUserData for LuaFileDescriptorProto {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("descriptor_dyn", |_, this, ()| {
            let descriptor: LuaMessageDescriptor = From::from(this.descriptor_dyn());
            Ok(descriptor)
        })
    }
}