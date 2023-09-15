use std::ops::{Deref, DerefMut};
use mlua::prelude::LuaUserData;
use mlua::UserDataFields;
use protobuf::descriptor::EnumValueDescriptorProto;

#[derive(PartialEq, Clone, Default, Debug)]
pub struct LuaEnumValueDescriptorProto(EnumValueDescriptorProto);

impl Deref for LuaEnumValueDescriptorProto {
    type Target = EnumValueDescriptorProto;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaEnumValueDescriptorProto {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<EnumValueDescriptorProto> for LuaEnumValueDescriptorProto {
    fn from(value: EnumValueDescriptorProto) -> Self {
        LuaEnumValueDescriptorProto(value)
    }
}

impl LuaUserData for LuaEnumValueDescriptorProto {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("name", |_, this| {
            Ok(this.name.clone())
        });
        fields.add_field_method_get("number", |_, this| {
            Ok(this.number)
        });
    }
}