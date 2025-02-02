use crate::descriptor::enum_descriptor::LuaEnumDescriptor;
use crate::message_dyn::LuaMessageDyn;
use crate::runtime_type::LuaRuntimeType;
use derive_more::{Deref, From, Into};
use mlua::prelude::LuaUserData;
use mlua::{Integer, UserDataMethods};
use protobuf::reflect::ReflectValueBox;

#[derive(Debug, Clone, Deref, From, Into)]
pub struct LuaReflectedValueBox(pub ReflectValueBox);

impl LuaUserData for LuaReflectedValueBox {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("get_type", |_, this, ()| {
            Ok::<LuaRuntimeType, _>(this.get_type().into())
        });

        methods.add_method("get_u32", |_, this, ()| {
            if let ReflectValueBox::U32(v) = this.0 {
                Ok(Some(v))
            } else {
                Ok(None)
            }
        });

        methods.add_method("get_u64", |_, this, ()| {
            if let ReflectValueBox::U64(v) = this.0 {
                Ok(Some(v))
            } else {
                Ok(None)
            }
        });

        methods.add_method("get_i32", |_, this, ()| {
            if let ReflectValueBox::I32(v) = this.0 {
                Ok(Some(v))
            } else {
                Ok(None)
            }
        });

        methods.add_method("get_i64", |_, this, ()| {
            if let ReflectValueBox::I64(v) = this.0 {
                Ok(Some(v))
            } else {
                Ok(None)
            }
        });

        methods.add_method("get_f32", |_, this, ()| {
            if let ReflectValueBox::F32(v) = this.0 {
                Ok(Some(v))
            } else {
                Ok(None)
            }
        });

        methods.add_method("get_f64", |_, this, ()| {
            if let ReflectValueBox::F64(v) = this.0 {
                Ok(Some(v))
            } else {
                Ok(None)
            }
        });

        methods.add_method("get_bool", |_, this, ()| {
            if let ReflectValueBox::Bool(v) = this.0 {
                Ok(Some(v))
            } else {
                Ok(None)
            }
        });

        methods.add_method("get_string", |_, this, ()| {
            if let ReflectValueBox::String(v) = &this.0 {
                Ok(Some(v.clone()))
            } else {
                Ok(None)
            }
        });

        methods.add_method("get_bytes", |_, this, ()| {
            if let ReflectValueBox::Bytes(v) = &this.0 {
                Ok(Some(v.clone()))
            } else {
                Ok(None)
            }
        });

        methods.add_method("get_enum", |lua, this, ()| {
            if let ReflectValueBox::Enum(e, i) = &this.0 {
                let mut mv = mlua::MultiValue::new();
                mv.push_back(mlua::Value::UserData(lua.create_any_userdata(LuaEnumDescriptor(e.clone()))?));
                mv.push_back(mlua::Value::Integer(Integer::from(*i)));
                Ok(mv)
            } else {
                Ok(mlua::MultiValue::new()) // 空多返回值，Lua 接收 nil
            }
        });

        methods.add_method("get_message", |_, this, ()| {
            if let ReflectValueBox::Message(m) = &this.0 {
                Ok(Some(LuaMessageDyn(m.clone_box())))
            } else {
                Ok(None)
            }
        });
    }
}