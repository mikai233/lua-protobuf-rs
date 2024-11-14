use mlua::prelude::LuaUserData;
use mlua::UserDataFields;
use protobuf::descriptor::EnumValueDescriptorProto;

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
pub struct LuaEnumValueDescriptorProto(EnumValueDescriptorProto);

impl LuaUserData for LuaEnumValueDescriptorProto {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("name", |_, this| Ok(this.name.clone()));
        fields.add_field_method_get("number", |_, this| Ok(this.number));
    }
}
