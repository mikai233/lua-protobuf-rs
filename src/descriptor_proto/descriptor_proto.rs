use crate::descriptor_proto::enum_descriptor_proto::LuaEnumDescriptorProto;
use crate::descriptor_proto::extension_range::LuaExtensionRange;
use crate::descriptor_proto::field_descriptor_proto::LuaFieldDescriptorProto;
use crate::descriptor_proto::oneof_descriptor_proto::LuaOneofDescriptorProto;
use crate::descriptor_proto::reserved_range::LuaReservedRange;
use crate::{add_message_dyn_trait_method, add_message_full_trait_method, add_message_trait_method};
use derive_more::{Deref, DerefMut, From, Into};
use mlua::prelude::LuaUserData;
use mlua::{MetaMethod, UserDataFields, UserDataMethods};
use protobuf::descriptor::DescriptorProto;

#[derive(PartialEq, Clone, Default, Debug, Deref, DerefMut, From, Into)]
pub struct LuaDescriptorProto(pub DescriptorProto);

impl LuaUserData for LuaDescriptorProto {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("name", |_, this| {
            Ok(this.name.clone())
        });
        
        fields.add_field_method_get("field", |_, this| {
            let field: Vec<LuaFieldDescriptorProto> = this.field.iter().map(Clone::clone).map(From::from).collect();
            Ok(field)
        });

        fields.add_field_method_get("extension", |_, this| {
            let extension: Vec<LuaFieldDescriptorProto> = this.extension.iter().map(Clone::clone).map(From::from).collect();
            Ok(extension)
        });

        fields.add_field_method_get("nested_type", |_, this| {
            let nested_type: Vec<LuaDescriptorProto> = this.nested_type.iter().map(Clone::clone).map(From::from).collect();
            Ok(nested_type)
        });

        fields.add_field_method_get("enum_type", |_, this| {
            let enum_type: Vec<LuaEnumDescriptorProto> = this.enum_type.iter().map(Clone::clone).map(From::from).collect();
            Ok(enum_type)
        });

        fields.add_field_method_get("extension_range", |_, this| {
            let extension_range: Vec<LuaExtensionRange> = this.extension_range.iter().map(Clone::clone).map(From::from).collect();
            Ok(extension_range)
        });

        fields.add_field_method_get("oneof_decl", |_, this| {
            let oneof_decl: Vec<LuaOneofDescriptorProto> = this.oneof_decl.iter().map(Clone::clone).map(From::from).collect();
            Ok(oneof_decl)
        });

        fields.add_field_method_get("reserved_range", |_, this| {
            let reserved_range: Vec<LuaReservedRange> = this.reserved_range.iter().map(Clone::clone).map(From::from).collect();
            Ok(reserved_range)
        });

        fields.add_field_method_get("reserved_name", |_, this| {
            Ok(this.reserved_name.clone())
        });
    }

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
            Ok(this.to_string())
        });

        methods.add_method("name", |_, this, ()| {
            Ok(this.name().to_string())
        });

        methods.add_method_mut("clear_name", |_, this, ()| {
            Ok(this.clear_name())
        });

        methods.add_method_mut("set_name", |_, this, name: String| {
            Ok(this.set_name(name))
        });

        methods.add_method_mut("mut_name", |_, this, ()| {
            Ok(this.mut_name().clone())
        });

        methods.add_method_mut("take_name", |_, this, ()| {
            Ok(this.take_name())
        });

        add_message_trait_method!(methods, DescriptorProto, LuaDescriptorProto);

        add_message_dyn_trait_method!(methods, DescriptorProto, LuaDescriptorProto);

        add_message_full_trait_method!(methods, DescriptorProto, LuaDescriptorProto);
    }
}