use std::ops::{Deref, DerefMut};

use mlua::prelude::LuaUserData;
use mlua::UserDataFields;
use protobuf::descriptor::descriptor_proto::ExtensionRange;

#[derive(PartialEq, Clone, Default, Debug)]
pub struct LuaExtensionRange(ExtensionRange);

impl Deref for LuaExtensionRange {
    type Target = ExtensionRange;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaExtensionRange {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl LuaUserData for LuaExtensionRange {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("start", |_, this| {
            Ok(this.start)
        });
        fields.add_field_method_get("end", |_, this| {
            Ok(this.end)
        });
        fields.add_field_method_get("options", |_, this| {
            Ok(0)
        });
    }
}