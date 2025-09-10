use crate::descriptor_proto::name_part::LuaNamePart;
use crate::{
    add_message_dyn_trait_method, add_message_full_trait_method, add_message_trait_method,
};
use derive_more::{Deref, DerefMut, From, Into};
use mlua::prelude::LuaUserData;
use mlua::{MetaMethod, UserDataFields, UserDataMethods};
use protobuf::descriptor::UninterpretedOption;

#[derive(PartialEq, Clone, Default, Debug, Deref, DerefMut, From, Into)]
pub struct LuaUninterpretedOption(pub UninterpretedOption);

impl LuaUserData for LuaUninterpretedOption {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("name", |_, this| {
            let name = this
                .name
                .iter()
                .map(Clone::clone)
                .map(Into::into)
                .collect::<Vec<LuaNamePart>>();
            Ok(name)
        });

        fields.add_field_method_get("identifier_value", |_, this| {
            Ok(this.identifier_value.clone())
        });

        fields.add_field_method_get("positive_int_value", |_, this| {
            Ok(this.positive_int_value.clone())
        });

        fields.add_field_method_get("negative_int_value", |_, this| {
            Ok(this.negative_int_value.clone())
        });

        fields.add_field_method_get("double_value", |_, this| Ok(this.double_value.clone()));

        fields.add_field_method_get("string_value", |_, this| {
            Ok(this
                .string_value
                .as_ref()
                .map(|v| String::from_utf8_lossy(v).to_string()))
        });

        fields.add_field_method_get("aggregate_value", |_, this| {
            Ok(this.aggregate_value.clone())
        });
    }

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| Ok(this.to_string()));

        methods.add_method("identifier_value", |_, this, ()| {
            Ok(this.identifier_value().to_string())
        });

        methods.add_method("identifier_value", |_, this, ()| {
            Ok(this.identifier_value().to_string())
        });

        methods.add_method_mut("clear_identifier_value", |_, this, ()| {
            this.clear_identifier_value();
            Ok(())
        });

        methods.add_method("has_identifier_value", |_, this, ()| {
            Ok(this.has_identifier_value())
        });

        methods.add_method_mut("set_identifier_value", |_, this, value: String| {
            this.set_identifier_value(value);
            Ok(())
        });

        methods.add_method_mut("mut_identifier_value", |_, this, ()| {
            Ok(this.mut_identifier_value().clone())
        });

        methods.add_method_mut("take_identifier_value", |_, this, ()| {
            Ok(this.take_identifier_value())
        });

        methods.add_method("positive_int_value", |_, this, ()| {
            Ok(this.positive_int_value())
        });

        methods.add_method_mut("clear_positive_int_value", |_, this, ()| {
            this.clear_positive_int_value();
            Ok(())
        });

        methods.add_method("has_positive_int_value", |_, this, ()| {
            Ok(this.has_positive_int_value())
        });

        methods.add_method_mut("set_positive_int_value", |_, this, value: u64| {
            this.set_positive_int_value(value);
            Ok(())
        });

        methods.add_method("negative_int_value", |_, this, ()| {
            Ok(this.negative_int_value())
        });

        methods.add_method_mut("clear_negative_int_value", |_, this, ()| {
            this.clear_negative_int_value();
            Ok(())
        });

        methods.add_method("has_negative_int_value", |_, this, ()| {
            Ok(this.has_negative_int_value())
        });

        methods.add_method_mut("set_negative_int_value", |_, this, value: i64| {
            this.set_negative_int_value(value);
            Ok(())
        });

        methods.add_method("double_value", |_, this, ()| Ok(this.double_value()));

        methods.add_method_mut("clear_double_value", |_, this, ()| {
            this.clear_double_value();
            Ok(())
        });

        methods.add_method(
            "has_double_value",
            |_, this, ()| Ok(this.has_double_value()),
        );

        methods.add_method_mut("set_double_value", |_, this, value: f64| {
            this.set_double_value(value);
            Ok(())
        });

        methods.add_method("string_value", |_, this, ()| {
            Ok(this.string_value().to_vec())
        });

        methods.add_method_mut("clear_string_value", |_, this, ()| {
            this.clear_string_value();
            Ok(())
        });

        methods.add_method(
            "has_string_value",
            |_, this, ()| Ok(this.has_string_value()),
        );

        methods.add_method_mut("set_string_value", |_, this, value: Vec<u8>| {
            this.set_string_value(value);
            Ok(())
        });

        methods.add_method_mut("mut_string_value", |_, this, ()| {
            Ok(this.mut_string_value().clone())
        });

        methods.add_method_mut("take_string_value", |_, this, ()| {
            Ok(this.take_string_value())
        });

        methods.add_method("aggregate_value", |_, this, ()| {
            Ok(this.aggregate_value().to_string())
        });

        methods.add_method_mut("clear_aggregate_value", |_, this, ()| {
            this.clear_aggregate_value();
            Ok(())
        });

        methods.add_method("has_aggregate_value", |_, this, ()| {
            Ok(this.has_aggregate_value())
        });

        methods.add_method_mut("set_aggregate_value", |_, this, value: String| {
            this.set_aggregate_value(value);
            Ok(())
        });

        methods.add_method_mut("mut_aggregate_value", |_, this, ()| {
            Ok(this.mut_aggregate_value().clone())
        });

        methods.add_method_mut("take_aggregate_value", |_, this, ()| {
            Ok(this.take_aggregate_value())
        });

        add_message_trait_method!(methods, UninterpretedOption, LuaUninterpretedOption);

        add_message_dyn_trait_method!(methods, UninterpretedOption, LuaUninterpretedOption);

        add_message_full_trait_method!(methods, UninterpretedOption, LuaUninterpretedOption);
    }
}
