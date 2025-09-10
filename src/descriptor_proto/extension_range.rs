use crate::descriptor_proto::extension_options::LuaExtensionRangeOptions;
use crate::{
    add_message_dyn_trait_method, add_message_full_trait_method, add_message_trait_method,
};
use derive_more::{Deref, DerefMut, From, Into};
use mlua::prelude::LuaUserData;
use mlua::{MetaMethod, UserDataFields, UserDataMethods};
use protobuf::descriptor::descriptor_proto::ExtensionRange;

#[derive(PartialEq, Clone, Default, Debug, Deref, DerefMut, From, Into)]
pub struct LuaExtensionRange(pub ExtensionRange);

impl LuaUserData for LuaExtensionRange {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("start", |_, this| Ok(this.start));
        fields.add_field_method_get("end", |_, this| Ok(this.end));
        fields.add_field_method_get("options", |_, this| {
            Ok::<Option<LuaExtensionRangeOptions>, _>(
                this.options.clone().into_option().map(Into::into),
            )
        });
    }

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| Ok(this.to_string()));

        methods.add_method("start", |_, this, ()| Ok(this.start()));

        methods.add_method("has_start", |_, this, ()| Ok(this.has_start()));

        methods.add_method_mut("clear_start", |_, this, ()| {
            this.clear_start();
            Ok(())
        });

        methods.add_method_mut("set_start", |_, this, value: i32| {
            this.set_start(value);
            Ok(())
        });

        methods.add_method("end", |_, this, ()| Ok(this.end()));

        methods.add_method("has_end", |_, this, ()| Ok(this.has_end()));

        methods.add_method_mut("clear_end", |_, this, ()| {
            this.clear_end();
            Ok(())
        });

        methods.add_method_mut("set_end", |_, this, value: i32| {
            this.set_end(value);
            Ok(())
        });

        add_message_trait_method!(methods, ExtensionRange, LuaExtensionRange);

        add_message_dyn_trait_method!(methods, ExtensionRange, LuaExtensionRange);

        add_message_full_trait_method!(methods, ExtensionRange, LuaExtensionRange);
    }
}
