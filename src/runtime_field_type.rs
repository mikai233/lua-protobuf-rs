use crate::runtime_type::LuaRuntimeType;
use derive_more::{Deref, From, Into};
use mlua::UserDataMethods;
use mlua::prelude::LuaUserData;
use protobuf::reflect::RuntimeFieldType;
use std::ops::Deref;

#[derive(Deref, From, Into)]
pub struct LuaRuntimeFieldType(pub RuntimeFieldType);

impl LuaUserData for LuaRuntimeFieldType {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("get_singular", |_, this, ()| {
            if let RuntimeFieldType::Singular(ty) = this.deref() {
                let ty: LuaRuntimeType = From::from(ty.clone());
                Ok(Some(ty))
            } else {
                Ok(None)
            }
        });

        methods.add_method("get_repeated", |_, this, ()| {
            if let RuntimeFieldType::Repeated(ty) = this.deref() {
                let ty: LuaRuntimeType = From::from(ty.clone());
                Ok(Some(ty))
            } else {
                Ok(None)
            }
        });

        methods.add_method("get_map", |_, this, ()| {
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
