use std::ops::{Deref, DerefMut};
use mlua::prelude::LuaUserData;
use protobuf::descriptor::DescriptorProto;

pub struct LuaDescriptorProto(DescriptorProto);

impl Deref for LuaDescriptorProto {
    type Target = DescriptorProto;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaDescriptorProto {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<DescriptorProto> for LuaDescriptorProto {
    fn from(value: DescriptorProto) -> Self {
        LuaDescriptorProto(value)
    }
}

impl LuaUserData for LuaDescriptorProto {}