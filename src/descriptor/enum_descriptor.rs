use crate::descriptor::enum_value_descriptor::LuaEnumValueDescriptor;
use crate::descriptor::message_descriptor::LuaMessageDescriptor;
use crate::descriptor_proto::enum_descriptor_proto::LuaEnumDescriptorProto;
use derive_more::{Deref, From, Into};
use mlua::prelude::LuaUserData;
use mlua::{MetaMethod, UserDataMethods};
use protobuf::reflect::EnumDescriptor;

#[derive(Clone, Eq, PartialEq, Hash, Deref, From, Into)]
pub struct LuaEnumDescriptor(pub EnumDescriptor);

impl LuaUserData for LuaEnumDescriptor {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
            Ok(this.to_string())
        });

        methods.add_method("proto", |_, this, ()| {
            Ok::<LuaEnumDescriptorProto, _>(this.proto().clone().into())
        });

        methods.add_method("name", |_, this, ()| {
            Ok(this.name().to_string())
        });

        methods.add_method("full_name", |_, this, ()| {
            Ok(this.full_name().to_string())
        });

        methods.add_method("name_to_package", |_, this, ()| {
            Ok(this.name_to_package().to_string())
        });

        // methods.add_function("for_type", |_, ()| {
        //     Ok(LuaEnumDescriptor(EnumDescriptor::for_type::<E>()))
        // });

        methods.add_method("enclosing_message", |_, this, ()| {
            Ok(this.enclosing_message().map(LuaMessageDescriptor))
        });

        methods.add_method("values", |_, this, ()| {
            let values: Vec<_> = this.values().map(LuaEnumValueDescriptor).collect();
            Ok(values)
        });

        methods.add_method("value_by_name", |_, this, name: String| {
            Ok(this.value_by_name(&name).map(LuaEnumValueDescriptor))
        });

        methods.add_method("value_by_number", |_, this, number: i32| {
            Ok(this.value_by_number(number).map(LuaEnumValueDescriptor))
        });

        methods.add_method("value_by_index", |_, this, index: usize| {
            Ok(LuaEnumValueDescriptor(this.value_by_index(index)))
        });

        methods.add_method("default_value", |_, this, ()| {
            Ok(LuaEnumValueDescriptor(this.default_value()))
        });

        methods.add_method("value_by_number_or_default", |_, this, number: i32| {
            Ok(LuaEnumValueDescriptor(this.value_by_number_or_default(number)))
        });

        // methods.add_method("is", |_, this| {
        //     Ok(this.is::<E>())
        // });
    }
}