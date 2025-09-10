use crate::descriptor_proto::enum_value_options::LuaEnumValueOptions;
use crate::{
    add_message_dyn_trait_method, add_message_full_trait_method, add_message_trait_method,
};
use derive_more::{Deref, DerefMut, From, Into};
use mlua::prelude::LuaUserData;
use mlua::{MetaMethod, UserDataFields, UserDataMethods};
use protobuf::descriptor::EnumValueDescriptorProto;

#[derive(PartialEq, Clone, Default, Debug, Deref, DerefMut, From, Into)]
pub struct LuaEnumValueDescriptorProto(pub EnumValueDescriptorProto);

impl LuaUserData for LuaEnumValueDescriptorProto {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("name", |_, this| Ok(this.name.clone()));

        fields.add_field_method_get("number", |_, this| Ok(this.number));

        fields.add_field_method_get("options", |_, this| {
            let options: Option<LuaEnumValueOptions> =
                this.options.clone().into_option().map(Into::into);
            Ok(options)
        });
    }

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| Ok(this.to_string()));

        methods.add_method("name", |_, this, ()| Ok(this.name().to_string()));

        methods.add_method_mut("clear_name", |_, this, ()| {
            this.clear_name();
            Ok(())
        });

        methods.add_method("has_name", |_, this, ()| Ok(this.has_name()));

        methods.add_method_mut("set_name", |_, this, value: String| {
            this.set_name(value);
            Ok(())
        });

        methods.add_method_mut("mut_name", |_, this, ()| Ok(this.mut_name().clone()));

        methods.add_method_mut("take_name", |_, this, ()| Ok(this.take_name()));

        methods.add_method("number", |_, this, ()| Ok(this.number()));

        methods.add_method_mut("clear_number", |_, this, ()| {
            this.clear_number();
            Ok(())
        });

        methods.add_method("has_number", |_, this, ()| Ok(this.has_number()));

        methods.add_method_mut("set_number", |_, this, value: i32| {
            this.set_number(value);
            Ok(())
        });

        add_message_trait_method!(
            methods,
            EnumValueDescriptorProto,
            LuaEnumValueDescriptorProto
        );

        add_message_dyn_trait_method!(
            methods,
            EnumValueDescriptorProto,
            LuaEnumValueDescriptorProto
        );

        add_message_full_trait_method!(
            methods,
            EnumValueDescriptorProto,
            LuaEnumValueDescriptorProto
        );
    }
}
