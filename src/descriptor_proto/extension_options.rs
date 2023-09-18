use std::ops::{Deref, DerefMut};

use mlua::prelude::LuaUserData;
use mlua::UserDataFields;
use protobuf::descriptor::ExtensionRangeOptions;

#[derive(PartialEq, Clone, Default, Debug)]
pub(crate) struct LuaExtensionRangeOptions(ExtensionRangeOptions);

impl Deref for LuaExtensionRangeOptions {
    type Target = ExtensionRangeOptions;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaExtensionRangeOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<ExtensionRangeOptions> for LuaExtensionRangeOptions {
    fn from(value: ExtensionRangeOptions) -> Self {
        LuaExtensionRangeOptions(value)
    }
}

impl LuaUserData for LuaExtensionRangeOptions {
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("uninterpreted_option", |_, this| {
            Ok(0)
        });
        fields.add_field_method_get("special_fields", |_, this| {
            Ok(0)
        });
    }
}