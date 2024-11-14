use crate::runtime_type::LuaRuntimeType;
use mlua::prelude::LuaUserData;
use mlua::UserDataFields;
use protobuf::reflect::RuntimeFieldType;
use std::ops::{Deref, DerefMut};

pub struct LuaRuntimeFieldType(RuntimeFieldType);

impl Deref for LuaRuntimeFieldType {
    type Target = RuntimeFieldType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaRuntimeFieldType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<RuntimeFieldType> for LuaRuntimeFieldType {
    fn from(value: RuntimeFieldType) -> Self {
        LuaRuntimeFieldType(value)
    }
}

impl LuaUserData for LuaRuntimeFieldType {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("singular", |_, this| {
            if let RuntimeFieldType::Singular(ty) = this.deref() {
                let ty: LuaRuntimeType = From::from(ty.clone());
                Ok(Some(ty))
            } else {
                Ok(None)
            }
        });
        fields.add_field_method_get("repeated", |_, this| {
            if let RuntimeFieldType::Repeated(ty) = this.deref() {
                let ty: LuaRuntimeType = From::from(ty.clone());
                Ok(Some(ty))
            } else {
                Ok(None)
            }
        });
        fields.add_field_method_get("map", |_, this| {
            if let RuntimeFieldType::Map(key_ty, value_ty) = this.deref() {
                let key: LuaRuntimeType = From::from(key_ty.clone());
                let value: LuaRuntimeType = From::from(value_ty.clone());
                Ok(Some(vec![key, value]))
            } else {
                Ok(None)
            }
        });
    }
}
