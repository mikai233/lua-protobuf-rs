use crate::descriptor_proto::enum_options::LuaEnumOptions;
use crate::descriptor_proto::enum_reserved_range::LuaEnumReservedRange;
use crate::descriptor_proto::enum_value_descriptor_proto::LuaEnumValueDescriptorProto;
use crate::{
    add_message_dyn_trait_method, add_message_full_trait_method, add_message_trait_method,
};
use derive_more::{Deref, DerefMut, From};
use mlua::prelude::LuaUserData;
use mlua::{MetaMethod, UserDataFields, UserDataMethods};
use protobuf::descriptor::EnumDescriptorProto;

#[derive(PartialEq, Clone, Default, Debug, From, Deref, DerefMut)]
pub struct LuaEnumDescriptorProto(pub EnumDescriptorProto);

impl LuaUserData for LuaEnumDescriptorProto {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("name", |_, this| Ok(this.name.clone()));

        fields.add_field_method_get("value", |_, this| {
            let value: Vec<LuaEnumValueDescriptorProto> =
                this.value.clone().into_iter().map(Into::into).collect();
            Ok(value)
        });

        fields.add_field_method_get("options", |_, this| {
            let options: Option<LuaEnumOptions> =
                this.options.clone().into_option().map(Into::into);
            Ok(options)
        });

        fields.add_field_method_get("reserved_range", |_, this| {
            let reserved_range = this
                .reserved_range
                .iter()
                .map(Clone::clone)
                .map(Into::into)
                .collect::<Vec<LuaEnumReservedRange>>();
            Ok(reserved_range)
        });

        fields.add_field_method_get("reserved_name", |_, this| Ok(this.reserved_name.clone()));
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

        add_message_trait_method!(methods, EnumDescriptorProto, LuaEnumDescriptorProto);

        add_message_dyn_trait_method!(methods, EnumDescriptorProto, LuaEnumDescriptorProto);

        add_message_full_trait_method!(methods, EnumDescriptorProto, LuaEnumDescriptorProto);
    }
}
