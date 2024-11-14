use mlua::prelude::LuaUserData;
use mlua::{UserDataFields, UserDataMethods};
use protobuf::descriptor::DescriptorProto;
use protobuf::MessageDyn;

use crate::descriptor::message_descriptor::LuaMessageDescriptor;
use crate::descriptor_proto::enum_descriptor_proto::LuaEnumDescriptorProto;
use crate::descriptor_proto::field_descriptor_proto::LuaFieldDescriptorProto;

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
pub struct LuaDescriptorProto(DescriptorProto);

impl LuaUserData for LuaDescriptorProto {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("name", |_, this| Ok(this.name.clone()));
        fields.add_field_method_get("field", |_, this| {
            let field: Vec<LuaFieldDescriptorProto> = this
                .field
                .iter()
                .map(Clone::clone)
                .map(From::from)
                .collect();
            Ok(field)
        });
        fields.add_field_method_get("extension", |_, this| {
            let extension: Vec<LuaFieldDescriptorProto> = this
                .extension
                .iter()
                .map(Clone::clone)
                .map(From::from)
                .collect();
            Ok(extension)
        });
        fields.add_field_method_get("nested_type", |_, this| {
            let nested_type: Vec<LuaDescriptorProto> = this
                .nested_type
                .iter()
                .map(Clone::clone)
                .map(From::from)
                .collect();
            Ok(nested_type)
        });
        fields.add_field_method_get("enum_type", |_, this| {
            let enum_type: Vec<LuaEnumDescriptorProto> = this
                .enum_type
                .iter()
                .map(Clone::clone)
                .map(From::from)
                .collect();
            Ok(enum_type)
        });
    }

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("descriptor_dyn", |_, this, ()| {
            let descriptor: LuaMessageDescriptor = From::from(this.descriptor_dyn());
            Ok(descriptor)
        })
    }
}
