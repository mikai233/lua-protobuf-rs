use crate::descriptor_proto::uninterpreted_option::LuaUninterpretedOption;
use crate::{add_message_dyn_trait_method, add_message_full_trait_method, add_message_trait_method};
use derive_more::{Deref, DerefMut, From, Into};
use mlua::prelude::LuaUserData;
use mlua::{MetaMethod, UserDataFields, UserDataMethods};
use protobuf::descriptor::EnumOptions;

#[derive(PartialEq, Clone, Default, Debug, Deref, DerefMut, From, Into)]
pub struct LuaEnumOptions(pub EnumOptions);

impl LuaUserData for LuaEnumOptions {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("allow_alias", |_, this| {
            Ok(this.allow_alias)
        });

        fields.add_field_method_get("deprecated", |_, this| {
            Ok(this.deprecated)
        });

        fields.add_field_method_get("uninterpreted_option", |_, this| {
            let uninterpreted_option = this.uninterpreted_option.iter().map(Clone::clone).map(Into::into).collect::<Vec<LuaUninterpretedOption>>();
            Ok(uninterpreted_option)
        })
    }

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
            Ok(this.to_string())
        });

        methods.add_method("allow_alias", |_, this, ()| {
            Ok(this.allow_alias())
        });

        methods.add_method_mut("clear_allow_alias", |_, this, ()| {
            this.clear_allow_alias();
            Ok(())
        });

        methods.add_method("has_allow_alias", |_, this, ()| {
            Ok(this.has_allow_alias())
        });

        methods.add_method_mut("set_allow_alias", |_, this, value: bool| {
            this.set_allow_alias(value);
            Ok(())
        });

        methods.add_method("deprecated", |_, this, ()| {
            Ok(this.deprecated())
        });

        methods.add_method_mut("clear_deprecated", |_, this, ()| {
            this.clear_deprecated();
            Ok(())
        });

        methods.add_method("has_deprecated", |_, this, ()| {
            Ok(this.has_deprecated())
        });

        methods.add_method_mut("set_deprecated", |_, this, value: bool| {
            this.set_deprecated(value);
            Ok(())
        });

        add_message_trait_method!(methods, EnumOptions, LuaEnumOptions);

        add_message_dyn_trait_method!(methods, EnumOptions, LuaEnumOptions);

        add_message_full_trait_method!(methods, EnumOptions, LuaEnumOptions);
    }
}