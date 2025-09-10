use crate::{
    add_message_dyn_trait_method, add_message_full_trait_method, add_message_trait_method,
};
use derive_more::{Deref, DerefMut, From, Into};
use mlua::prelude::LuaUserData;
use mlua::{MetaMethod, UserDataFields, UserDataMethods};
use protobuf::descriptor::source_code_info::Location;

#[derive(PartialEq, Clone, Default, Debug, Deref, DerefMut, From, Into)]
pub struct LuaLocation(pub Location);

impl LuaUserData for LuaLocation {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("path", |_, this| Ok(this.path.clone()));

        fields.add_field_method_get("span", |_, this| Ok(this.span.clone()));

        fields.add_field_method_get("leading_comments", |_, this| {
            Ok(this.leading_comments.clone())
        });

        fields.add_field_method_get("trailing_comments", |_, this| {
            Ok(this.trailing_comments.clone())
        });

        fields.add_field_method_get("leading_detached_comments", |_, this| {
            Ok(this.leading_detached_comments.clone())
        });
    }

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| Ok(this.to_string()));

        methods.add_method("leading_comments", |_, this, ()| {
            Ok(this.leading_comments().to_string())
        });

        methods.add_method_mut("clear_leading_comments", |_, this, ()| {
            this.clear_leading_comments();
            Ok(())
        });

        methods.add_method("has_leading_comments", |_, this, ()| {
            Ok(this.has_leading_comments())
        });

        methods.add_method_mut(
            "set_leading_comments",
            |_, this, v: ::std::string::String| {
                this.set_leading_comments(v);
                Ok(())
            },
        );

        methods.add_method_mut("mut_leading_comments", |_, this, ()| {
            Ok(this.mut_leading_comments().clone())
        });

        methods.add_method_mut("take_leading_comments", |_, this, ()| {
            Ok(this.take_leading_comments())
        });

        methods.add_method("trailing_comments", |_, this, ()| {
            Ok(this.trailing_comments().to_string())
        });

        methods.add_method_mut("clear_trailing_comments", |_, this, ()| {
            this.clear_trailing_comments();
            Ok(())
        });

        methods.add_method("has_trailing_comments", |_, this, ()| {
            Ok(this.has_trailing_comments())
        });

        methods.add_method_mut("set_trailing_comments", |_, this, v: String| {
            this.set_trailing_comments(v);
            Ok(())
        });

        methods.add_method_mut("mut_trailing_comments", |_, this, ()| {
            Ok(this.mut_trailing_comments().clone())
        });

        methods.add_method_mut("take_trailing_comments", |_, this, ()| {
            Ok(this.take_trailing_comments())
        });

        add_message_trait_method!(methods, Location, LuaLocation);

        add_message_dyn_trait_method!(methods, Location, LuaLocation);

        add_message_full_trait_method!(methods, Location, LuaLocation);
    }
}
