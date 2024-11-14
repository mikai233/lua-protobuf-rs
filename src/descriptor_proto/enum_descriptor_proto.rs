use crate::descriptor_proto::enum_value_descriptor_proto::LuaEnumValueDescriptorProto;
use mlua::prelude::LuaUserData;
use mlua::UserDataFields;
use protobuf::descriptor::EnumDescriptorProto;

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
pub struct LuaEnumDescriptorProto(EnumDescriptorProto);

impl LuaUserData for LuaEnumDescriptorProto {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("name", |_, this| Ok(this.name.clone()));
        fields.add_field_method_get("value", |_, this| {
            let value: Vec<LuaEnumValueDescriptorProto> =
                this.value.clone().into_iter().map(Into::into).collect();
            Ok(value)
        });
    }
}
