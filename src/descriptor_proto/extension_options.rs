use mlua::prelude::LuaUserData;
use mlua::UserDataFields;
use protobuf::descriptor::ExtensionRangeOptions;

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
pub(crate) struct LuaExtensionRangeOptions(ExtensionRangeOptions);

impl LuaUserData for LuaExtensionRangeOptions {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("uninterpreted_option", |_, this| Ok(0));
        fields.add_field_method_get("special_fields", |_, this| Ok(0));
    }
}
