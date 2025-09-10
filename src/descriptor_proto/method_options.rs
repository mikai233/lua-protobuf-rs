use crate::descriptor_proto::uninterpreted_option::LuaUninterpretedOption;
use crate::{
    add_message_dyn_trait_method, add_message_full_trait_method, add_message_trait_method,
};
use anyhow::anyhow;
use derive_more::{Deref, DerefMut, From, Into};
use mlua::prelude::LuaUserData;
use mlua::{MetaMethod, UserDataFields, UserDataMethods};
use protobuf::Enum;
use protobuf::descriptor::MethodOptions;
use protobuf::descriptor::method_options::IdempotencyLevel;

#[derive(PartialEq, Clone, Default, Debug, Deref, DerefMut, From, Into)]
pub struct LuaMethodOptions(pub MethodOptions);

impl LuaUserData for LuaMethodOptions {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("deprecated", |_, this| {
            Ok(this.deprecated.unwrap_or(false)) // 默认返回 false，如果没有设置
        });

        fields.add_field_method_get("idempotency_level", |_, this| {
            Ok(this
                .idempotency_level
                .as_ref()
                .map(|e| e.enum_value_or_default() as i32)) // 可能是 Option<EnumOrUnknown>
        });

        fields.add_field_method_get("uninterpreted_option", |_, this| {
            let uninterpreted_option = this
                .uninterpreted_option
                .iter()
                .map(Clone::clone)
                .map(Into::into)
                .collect::<Vec<LuaUninterpretedOption>>();
            Ok(uninterpreted_option)
        });
    }

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| Ok(this.to_string()));

        methods.add_method("deprecated", |_, this, ()| Ok(this.deprecated()));

        methods.add_method_mut(
            "clear_deprecated",
            |_, this, ()| Ok(this.clear_deprecated()),
        );

        methods.add_method("has_deprecated", |_, this, ()| Ok(this.has_deprecated()));

        methods.add_method_mut("set_deprecated", |_, this, v: bool| {
            Ok(this.set_deprecated(v))
        });

        methods.add_method("idempotency_level", |_, this, ()| {
            Ok(this.idempotency_level() as i32)
        });

        methods.add_method_mut("clear_idempotency_level", |_, this, ()| {
            Ok(this.clear_idempotency_level())
        });

        methods.add_method("has_idempotency_level", |_, this, ()| {
            Ok(this.has_idempotency_level())
        });

        methods.add_method_mut("set_idempotency_level", |_, this, v: i32| {
            let level =
                IdempotencyLevel::from_i32(v).ok_or(anyhow!("unknown idempotency_level {v}"))?;
            Ok(this.set_idempotency_level(level))
        });

        add_message_trait_method!(methods, MethodOptions, LuaMethodOptions);

        add_message_dyn_trait_method!(methods, MethodOptions, LuaMethodOptions);

        add_message_full_trait_method!(methods, MethodOptions, LuaMethodOptions);
    }
}
