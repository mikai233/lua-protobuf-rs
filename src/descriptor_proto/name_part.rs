use crate::{add_message_dyn_trait_method, add_message_full_trait_method, add_message_trait_method};
use derive_more::{Deref, DerefMut, From, Into};
use mlua::prelude::LuaUserData;
use mlua::{MetaMethod, UserDataFields, UserDataMethods};
use protobuf::descriptor::uninterpreted_option::NamePart;

#[derive(PartialEq, Clone, Default, Debug, Deref, DerefMut, From, Into)]
pub struct LuaNamePart(pub NamePart);

impl LuaUserData for LuaNamePart {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("name_part", |_, this| {
            Ok(this.name_part.clone())
        });

        fields.add_field_method_get("is_extension", |_, this| {
            Ok(this.is_extension)
        });
    }

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
            Ok(this.to_string())
        });

        methods.add_method("name_part", |_, this, ()| {
            Ok(this.name_part().to_string())
        });

        methods.add_method_mut("clear_name_part", |_, this, ()| {
            Ok(this.clear_name_part())
        });

        methods.add_method_mut("set_name_part", |_, this, v: String| {
            Ok(this.set_name_part(v))
        });

        methods.add_method_mut("mut_name_part", |_, this, ()| {
            Ok(this.mut_name_part().clone())
        });

        methods.add_method_mut("take_name_part", |_, this, ()| {
            Ok(this.take_name_part())
        });

        methods.add_method("is_extension", |_, this, ()| {
            Ok(this.is_extension())
        });

        methods.add_method_mut("clear_is_extension", |_, this, ()| {
            Ok(this.clear_is_extension())
        });

        methods.add_method("has_is_extension", |_, this, ()| {
            Ok(this.has_is_extension())
        });

        methods.add_method_mut("set_is_extension", |_, this, v: bool| {
            Ok(this.set_is_extension(v))
        });

        add_message_trait_method!(methods, NamePart, LuaNamePart);

        add_message_dyn_trait_method!(methods, NamePart, LuaNamePart);

        add_message_full_trait_method!(methods, NamePart, LuaNamePart);
    }
}