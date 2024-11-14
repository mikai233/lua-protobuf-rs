use std::ops::{Deref, DerefMut};

use mlua::prelude::LuaUserData;
use mlua::UserDataMethods;
use protobuf::reflect::EnumDescriptor;

use crate::descriptor::enum_value_descriptor::LuaEnumValueDescriptor;
use crate::descriptor::message_descriptor::LuaMessageDescriptor;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct LuaEnumDescriptor(EnumDescriptor);

impl Deref for LuaEnumDescriptor {
    type Target = EnumDescriptor;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaEnumDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<EnumDescriptor> for LuaEnumDescriptor {
    fn from(value: EnumDescriptor) -> Self {
        LuaEnumDescriptor(value)
    }
}

impl LuaUserData for LuaEnumDescriptor {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("name", |_, this, ()| Ok(this.name().to_string()));
        methods.add_method("full_name", |_, this, ()| Ok(this.full_name().to_string()));
        methods.add_method("name_to_package", |_, this, ()| {
            Ok(this.name_to_package().to_string())
        });
        methods.add_method("enclosing_message", |_, this, ()| {
            let enclosing_message: Option<LuaMessageDescriptor> =
                this.enclosing_message().map(From::from);
            Ok(enclosing_message)
        });
        methods.add_method("values", |_, this, ()| {
            let values: Vec<LuaEnumValueDescriptor> = this.values().map(From::from).collect();
            Ok(values)
        });
        methods.add_method("value_by_name", |_, this, name: String| {
            let descriptor: Option<LuaEnumValueDescriptor> =
                this.value_by_name(name.as_str()).map(From::from);
            Ok(descriptor)
        });
        methods.add_method("value_by_number", |_, this, number: i32| {
            let descriptor: Option<LuaEnumValueDescriptor> =
                this.value_by_number(number).map(From::from);
            Ok(descriptor)
        });
        methods.add_method("value_by_index", |_, this, index: usize| {
            let descriptor: LuaEnumValueDescriptor = this.value_by_index(index).into();
            Ok(descriptor)
        });
        methods.add_method("default_value", |_, this, ()| {
            let descriptor: LuaEnumValueDescriptor = this.default_value().into();
            Ok(descriptor)
        });
        methods.add_method("value_by_number_or_default", |_, this, number: i32| {
            let descriptor: LuaEnumValueDescriptor = this.value_by_number_or_default(number).into();
            Ok(descriptor)
        });
    }
}
