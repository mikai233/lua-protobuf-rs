use mlua::prelude::LuaUserData;
use mlua::UserDataFields;
use protobuf::descriptor::descriptor_proto::ExtensionRange;

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
pub struct LuaExtensionRange(ExtensionRange);

impl LuaUserData for LuaExtensionRange {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("start", |_, this| Ok(this.start));
        fields.add_field_method_get("end", |_, this| Ok(this.end));
        fields.add_field_method_get("options", |_, this| Ok(0));
    }
}
