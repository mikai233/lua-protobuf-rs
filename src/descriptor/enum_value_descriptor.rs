use crate::descriptor::enum_descriptor::LuaEnumDescriptor;
use crate::descriptor_proto::enum_value_descriptor_proto::LuaEnumValueDescriptorProto;
use derive_more::{Deref, From, Into};
use mlua::prelude::LuaUserData;
use mlua::{MetaMethod, UserDataMethods};
use protobuf::reflect::EnumValueDescriptor;

#[derive(Clone, Eq, PartialEq, Hash, Deref, From, Into)]
pub struct LuaEnumValueDescriptor(pub EnumValueDescriptor);

impl LuaUserData for LuaEnumValueDescriptor {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method(MetaMethod::ToString, |_, this, ()| {
            Ok(this.to_string())
        });

        methods.add_method("proto", |_, this, ()| {
            Ok::<LuaEnumValueDescriptorProto, _>(this.proto().clone().into())
        });

        methods.add_method("name", |_, this, ()| {
            Ok(this.name().to_string())
        });

        methods.add_method("full_name", |_, this, ()| {
            Ok(this.full_name())
        });

        methods.add_method("value", |_, this, ()| {
            Ok(this.value())
        });

        methods.add_method("enum_descriptor", |_, this, ()| {
            Ok(LuaEnumDescriptor(this.enum_descriptor().clone()))
        });

        // methods.add_method("cast", |_, this, ()| {
        //     Ok(this.cast::<E>())
        // });
    }
}