use std::ops::{Deref, DerefMut};
use mlua::prelude::LuaUserData;
use mlua::UserDataFields;
use protobuf::descriptor::EnumDescriptorProto;
use crate::descriptor_proto::enum_value_descriptor_proto::LuaEnumValueDescriptorProto;

#[derive(PartialEq, Clone, Default, Debug)]
pub struct LuaEnumDescriptorProto(EnumDescriptorProto);

impl Deref for LuaEnumDescriptorProto {
    type Target = EnumDescriptorProto;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaEnumDescriptorProto {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<EnumDescriptorProto> for LuaEnumDescriptorProto {
    fn from(value: EnumDescriptorProto) -> Self {
        LuaEnumDescriptorProto(value)
    }
}

impl LuaUserData for LuaEnumDescriptorProto {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("name", |_, this| {
            Ok(this.name.clone())
        });
        fields.add_field_method_get("value", |_, this| {
            let value: Vec<LuaEnumValueDescriptorProto> = this.value.clone().into_iter().map(Into::into).collect();
            Ok(value)
        });
    }
}