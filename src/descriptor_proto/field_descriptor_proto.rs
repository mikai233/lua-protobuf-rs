use crate::descriptor_proto::field_options::LuaFieldOptions;
use crate::{
    add_message_dyn_trait_method, add_message_full_trait_method, add_message_trait_method,
};
use anyhow::anyhow;
use derive_more::{Deref, DerefMut, From, Into};
use mlua::prelude::LuaUserData;
use mlua::{MetaMethod, UserDataFields, UserDataMethods};
use protobuf::Enum;
use protobuf::descriptor::FieldDescriptorProto;
use protobuf::descriptor::field_descriptor_proto::{Label, Type};

#[derive(PartialEq, Clone, Default, Debug, Deref, DerefMut, From, Into)]
pub struct LuaFieldDescriptorProto(pub FieldDescriptorProto);

impl LuaUserData for LuaFieldDescriptorProto {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("name", |_, this| Ok(this.name.clone()));

        fields.add_field_method_get("number", |_, this| Ok(this.number));

        fields.add_field_method_get("label", |_, this| {
            Ok(this
                .label
                .as_ref()
                .map(|e| e.enum_value_or_default() as i32))
        });

        fields.add_field_method_get("type_", |_, this| {
            Ok(this
                .type_
                .as_ref()
                .map(|e| e.enum_value_or_default() as i32))
        });

        fields.add_field_method_get("type_name", |_, this| Ok(this.type_name.clone()));

        fields.add_field_method_get("extendee", |_, this| Ok(this.extendee.clone()));

        fields.add_field_method_get("default_value", |_, this| Ok(this.default_value.clone()));

        fields.add_field_method_get("oneof_index", |_, this| Ok(this.oneof_index));

        fields.add_field_method_get("json_name", |_, this| Ok(this.json_name.clone()));

        fields.add_field_method_get("options", |_, this| {
            let options: Option<LuaFieldOptions> =
                this.options.clone().into_option().map(Into::into);
            Ok(options)
        });

        fields.add_field_method_get("proto3_optional", |_, this| Ok(this.proto3_optional));
    }

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| Ok(this.to_string()));

        methods.add_method("name", |_, this, ()| Ok(this.name().to_string()));

        methods.add_method("has_name", |_, this, ()| Ok(this.has_name()));

        methods.add_method_mut("clear_name", |_, this, ()| {
            this.clear_name();
            Ok(())
        });

        methods.add_method_mut("set_name", |_, this, value: String| {
            this.set_name(value);
            Ok(())
        });

        methods.add_method_mut("mut_name", |_, this, ()| {
            let name = this.mut_name();
            Ok(name.clone())
        });

        methods.add_method_mut("take_name", |_, this, ()| Ok(this.take_name()));

        methods.add_method("number", |_, this, ()| Ok(this.number()));

        methods.add_method("has_number", |_, this, ()| Ok(this.has_number()));

        methods.add_method_mut("clear_number", |_, this, ()| {
            this.clear_number();
            Ok(())
        });

        methods.add_method_mut("set_number", |_, this, value: i32| {
            this.set_number(value);
            Ok(())
        });

        methods.add_method("label", |_, this, ()| {
            Ok(this.label() as i32) // 假设 Label 是枚举类型，转换为 i32
        });

        methods.add_method("has_label", |_, this, ()| Ok(this.has_label()));

        methods.add_method_mut("clear_label", |_, this, ()| {
            this.clear_label();
            Ok(())
        });

        methods.add_method_mut("set_label", |_, this, value: i32| {
            let label = Label::from_i32(value).ok_or(anyhow!("unknown label {value}"))?;
            this.set_label(label);
            Ok(())
        });

        methods.add_method("type_", |_, this, ()| Ok(this.type_() as i32));

        methods.add_method("has_type", |_, this, ()| Ok(this.has_type()));

        methods.add_method_mut("clear_type_", |_, this, ()| {
            this.clear_type_();
            Ok(())
        });

        methods.add_method_mut("set_type", |_, this, value: i32| {
            let type_ = Type::from_i32(value).ok_or(anyhow!("unknown type {value}"))?;
            this.set_type(type_);
            Ok(())
        });

        methods.add_method("type_name", |_, this, ()| Ok(this.type_name().to_string()));

        methods.add_method("has_type_name", |_, this, ()| Ok(this.has_type_name()));

        methods.add_method_mut("clear_type_name", |_, this, ()| {
            this.clear_type_name();
            Ok(())
        });

        methods.add_method_mut("set_type_name", |_, this, value: String| {
            this.set_type_name(value);
            Ok(())
        });

        methods.add_method_mut("mut_type_name", |_, this, ()| {
            Ok(this.mut_type_name().clone())
        });

        methods.add_method_mut("take_type_name", |_, this, ()| Ok(this.take_type_name()));

        methods.add_method("extendee", |_, this, ()| Ok(this.extendee().to_string()));

        methods.add_method("has_extendee", |_, this, ()| Ok(this.has_extendee()));

        methods.add_method_mut("clear_extendee", |_, this, ()| {
            this.clear_extendee();
            Ok(())
        });

        methods.add_method_mut("set_extendee", |_, this, value: String| {
            this.set_extendee(value);
            Ok(())
        });

        methods.add_method_mut("mut_extendee", |_, this, ()| {
            Ok(this.mut_extendee().clone())
        });

        methods.add_method_mut("take_extendee", |_, this, ()| Ok(this.take_extendee()));

        methods.add_method("default_value", |_, this, ()| {
            Ok(this.default_value().to_string())
        });

        methods.add_method("has_default_value", |_, this, ()| {
            Ok(this.has_default_value())
        });

        methods.add_method_mut("clear_default_value", |_, this, ()| {
            this.clear_default_value();
            Ok(())
        });

        methods.add_method_mut("set_default_value", |_, this, value: String| {
            this.set_default_value(value);
            Ok(())
        });

        methods.add_method_mut("mut_default_value", |_, this, ()| {
            Ok(this.mut_default_value().clone())
        });

        methods.add_method_mut("take_default_value", |_, this, ()| {
            Ok(this.take_default_value())
        });

        methods.add_method("oneof_index", |_, this, ()| Ok(this.oneof_index()));

        methods.add_method("has_oneof_index", |_, this, ()| Ok(this.has_oneof_index()));

        methods.add_method_mut("clear_oneof_index", |_, this, ()| {
            this.clear_oneof_index();
            Ok(())
        });

        methods.add_method_mut("set_oneof_index", |_, this, value: i32| {
            this.set_oneof_index(value);
            Ok(())
        });

        methods.add_method("json_name", |_, this, ()| Ok(this.json_name().to_string()));

        methods.add_method("has_json_name", |_, this, ()| Ok(this.has_json_name()));

        methods.add_method_mut("clear_json_name", |_, this, ()| {
            this.clear_json_name();
            Ok(())
        });

        methods.add_method_mut("set_json_name", |_, this, value: String| {
            this.set_json_name(value);
            Ok(())
        });

        methods.add_method_mut("mut_json_name", |_, this, ()| {
            Ok(this.mut_json_name().clone())
        });

        methods.add_method_mut("take_json_name", |_, this, ()| Ok(this.take_json_name()));

        methods.add_method("proto3_optional", |_, this, ()| Ok(this.proto3_optional()));

        methods.add_method("has_proto3_optional", |_, this, ()| {
            Ok(this.has_proto3_optional())
        });

        methods.add_method_mut("clear_proto3_optional", |_, this, ()| {
            this.clear_proto3_optional();
            Ok(())
        });

        methods.add_method_mut("set_proto3_optional", |_, this, value: bool| {
            this.set_proto3_optional(value);
            Ok(())
        });

        add_message_trait_method!(methods, FieldDescriptorProto, LuaFieldDescriptorProto);

        add_message_dyn_trait_method!(methods, FieldDescriptorProto, LuaFieldDescriptorProto);

        add_message_full_trait_method!(methods, FieldDescriptorProto, LuaFieldDescriptorProto);
    }
}
