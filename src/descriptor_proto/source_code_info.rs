use crate::descriptor_proto::location::LuaLocation;
use crate::{add_message_dyn_trait_method, add_message_full_trait_method, add_message_trait_method};
use derive_more::{Deref, DerefMut, From, Into};
use mlua::prelude::LuaUserData;
use mlua::{MetaMethod, UserDataFields, UserDataMethods};
use protobuf::descriptor::SourceCodeInfo;

#[derive(PartialEq, Clone, Default, Debug, Deref, DerefMut, From, Into)]
pub struct LuaSourceCodeInfo(pub SourceCodeInfo);

impl LuaUserData for LuaSourceCodeInfo {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("location", |_, this| {
            let location = this.location.iter().map(Clone::clone).map(Into::into).collect::<Vec<LuaLocation>>();
            Ok(location)
        });
    }

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
            Ok(this.to_string())
        });

        add_message_trait_method!(methods, SourceCodeInfo, LuaSourceCodeInfo);

        add_message_dyn_trait_method!(methods, SourceCodeInfo, LuaSourceCodeInfo);

        add_message_full_trait_method!(methods, SourceCodeInfo, LuaSourceCodeInfo);
    }
}