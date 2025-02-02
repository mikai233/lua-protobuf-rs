use crate::descriptor_proto::uninterpreted_option::LuaUninterpretedOption;
use crate::{add_message_dyn_trait_method, add_message_full_trait_method, add_message_trait_method};
use anyhow::anyhow;
use derive_more::{Deref, DerefMut, From, Into};
use mlua::prelude::LuaUserData;
use mlua::{MetaMethod, UserDataFields, UserDataMethods};
use protobuf::descriptor::field_options::{CType, JSType};
use protobuf::descriptor::FieldOptions;
use protobuf::Enum;

#[derive(PartialEq, Clone, Default, Debug, Deref, DerefMut, From, Into)]
pub struct LuaFieldOptions(pub FieldOptions);

impl LuaUserData for LuaFieldOptions {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("ctype", |_, this| {
            Ok(this.ctype.as_ref().map(|e| e.enum_value_or_default() as i32))
        });

        fields.add_field_method_get("packed", |_, this| {
            Ok(this.packed)
        });

        fields.add_field_method_get("jstype", |_, this| {
            Ok(this.jstype.as_ref().map(|e| e.enum_value_or_default() as i32))
        });

        fields.add_field_method_get("lazy", |_, this| {
            Ok(this.lazy)
        });

        fields.add_field_method_get("deprecated", |_, this| {
            Ok(this.deprecated)
        });

        fields.add_field_method_get("weak", |_, this| {
            Ok(this.weak)
        });

        fields.add_field_method_get("uninterpreted_option", |_, this| {
            let uninterpreted_option = this.uninterpreted_option.iter().map(Clone::clone).map(Into::into).collect::<Vec<LuaUninterpretedOption>>();
            Ok(uninterpreted_option)
        });
    }

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
            Ok(this.to_string())
        });

        methods.add_method("ctype", |_, this, ()| {
            Ok(this.ctype() as i32)
        });

        methods.add_method_mut("clear_ctype", |_, this, ()| {
            this.clear_ctype();
            Ok(())
        });

        methods.add_method("has_ctype", |_, this, ()| {
            Ok(this.has_ctype())
        });

        methods.add_method_mut("set_ctype", |_, this, v: i32| {
            let v = CType::from_i32(v).ok_or(anyhow!("unknown ctype {v}"))?;
            this.set_ctype(v);
            Ok(())
        });

        methods.add_method("packed", |_, this, ()| {
            Ok(this.packed())
        });

        methods.add_method_mut("clear_packed", |_, this, ()| {
            this.clear_packed();
            Ok(())
        });

        methods.add_method("has_packed", |_, this, ()| {
            Ok(this.has_packed())
        });

        methods.add_method_mut("set_packed", |_, this, v: bool| {
            this.set_packed(v);
            Ok(())
        });

        methods.add_method("jstype", |_, this, ()| {
            Ok(this.jstype() as i32)
        });

        methods.add_method_mut("clear_jstype", |_, this, ()| {
            this.clear_jstype();
            Ok(())
        });

        methods.add_method("has_jstype", |_, this, ()| {
            Ok(this.has_jstype())
        });

        methods.add_method_mut("set_jstype", |_, this, v: i32| {
            let v = JSType::from_i32(v).ok_or(anyhow!("unknown jstype {v}"))?;
            this.set_jstype(v);
            Ok(())
        });

        methods.add_method("lazy", |_, this, ()| {
            Ok(this.lazy())
        });

        methods.add_method_mut("clear_lazy", |_, this, ()| {
            this.clear_lazy();
            Ok(())
        });

        methods.add_method("has_lazy", |_, this, ()| {
            Ok(this.has_lazy())
        });

        methods.add_method_mut("set_lazy", |_, this, v: bool| {
            this.set_lazy(v);
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

        methods.add_method_mut("set_deprecated", |_, this, v: bool| {
            this.set_deprecated(v);
            Ok(())
        });

        methods.add_method("weak", |_, this, ()| {
            Ok(this.weak())
        });

        methods.add_method_mut("clear_weak", |_, this, ()| {
            this.clear_weak();
            Ok(())
        });

        methods.add_method("has_weak", |_, this, ()| {
            Ok(this.has_weak())
        });

        methods.add_method_mut("set_weak", |_, this, v: bool| {
            this.set_weak(v);
            Ok(())
        });

        add_message_trait_method!(methods, FieldOptions, LuaFieldOptions);

        add_message_dyn_trait_method!(methods, FieldOptions, LuaFieldOptions);

        add_message_full_trait_method!(methods, FieldOptions, LuaFieldOptions);
    }
}