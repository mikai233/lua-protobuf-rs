use crate::descriptor_proto::uninterpreted_option::LuaUninterpretedOption;
use crate::{add_message_dyn_trait_method, add_message_full_trait_method, add_message_trait_method};
use derive_more::{Deref, DerefMut, From, Into};
use mlua::prelude::LuaUserData;
use mlua::{MetaMethod, UserDataFields, UserDataMethods};
use protobuf::descriptor::OneofOptions;

#[derive(PartialEq, Clone, Default, Debug, Deref, DerefMut, From, Into)]
pub(crate) struct LuaOneofOptions(pub OneofOptions);

impl LuaUserData for LuaOneofOptions {
    fn add_fields<'lua, F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("uninterpreted_option", |_, this| {
            let uninterpreted_option = this.uninterpreted_option.iter().map(Clone::clone).map(Into::into).collect::<Vec<LuaUninterpretedOption>>();
            Ok(uninterpreted_option)
        });
    }

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
            Ok(this.to_string())
        });

        add_message_trait_method!(methods, OneofOptions, LuaOneofOptions);

        add_message_dyn_trait_method!(methods, OneofOptions, LuaOneofOptions);

        add_message_full_trait_method!(methods, OneofOptions, LuaOneofOptions);
    }
}