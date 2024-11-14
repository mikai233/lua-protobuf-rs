use std::ops::{Deref, DerefMut};

use anyhow::anyhow;
use mlua::prelude::LuaUserData;
use mlua::{ExternalError, UserDataMethods};
use protobuf::reflect::FieldDescriptor;

use crate::descriptor::message_descriptor::LuaMessageDescriptor;
use crate::descriptor::oneof_descriptor::LuaOneofDescriptor;
use crate::runtime_field_type::LuaRuntimeFieldType;
use crate::runtime_type::LuaRuntimeType;

#[derive(Clone, Eq, PartialEq)]
pub struct LuaFieldDescriptor(FieldDescriptor);

impl Deref for LuaFieldDescriptor {
    type Target = FieldDescriptor;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaFieldDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<FieldDescriptor> for LuaFieldDescriptor {
    fn from(value: FieldDescriptor) -> Self {
        LuaFieldDescriptor(value)
    }
}

impl LuaUserData for LuaFieldDescriptor {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("name", |_, this, ()| Ok(this.name().to_string()));
        methods.add_method("number", |_, this, ()| Ok(this.number()));
        methods.add_method("full_name", |_, this, ()| Ok(this.full_name()));
        methods.add_method("containing_oneof_including_synthetic", |_, this, ()| {
            let descriptor: Option<LuaOneofDescriptor> =
                this.containing_oneof_including_synthetic().map(From::from);
            Ok(descriptor)
        });
        methods.add_method("containing_oneof", |_, this, ()| {
            let descriptor: Option<LuaOneofDescriptor> = this.containing_oneof().map(From::from);
            Ok(descriptor)
        });
        methods.add_method("containing_message", |_, this, ()| {
            let descriptor: LuaMessageDescriptor = this.containing_message().into();
            Ok(descriptor)
        });
        methods.add_method("json_name", |_, this, ()| Ok(this.json_name().to_string()));
        methods.add_method("is_singular", |_, this, ()| Ok(this.is_singular()));
        methods.add_method("is_required", |_, this, ()| Ok(this.is_required()));
        methods.add_method("is_repeated_or_map", |_, this, ()| {
            Ok(this.is_repeated_or_map())
        });
        methods.add_method("is_repeated", |_, this, ()| Ok(this.is_repeated()));
        methods.add_method("is_map", |_, this, ()| Ok(this.is_repeated()));
        methods.add_method("singular_runtime_type", |_, this, ()| {
            if this.is_singular() {
                let ty: LuaRuntimeType = this.singular_runtime_type().into();
                Ok(ty)
            } else {
                Err(anyhow!("{} is not singular", this.full_name()).into_lua_err())
            }
        });
        methods.add_method("runtime_field_type", |_, this, ()| {
            let ty: LuaRuntimeFieldType = this.runtime_field_type().into();
            Ok(ty)
        });
    }
}
