use crate::descriptor::message_descriptor::LuaMessageDescriptor;
use derive_more::{Deref, From, Into};
use mlua::prelude::LuaUserData;
use mlua::UserDataMethods;
use protobuf::MessageDyn;
use std::fmt::{Display, Formatter};

#[derive(Debug, Deref, From, Into)]
pub struct LuaMessageDyn(pub Box<dyn MessageDyn>);

impl Display for LuaMessageDyn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl LuaUserData for LuaMessageDyn {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("descriptor_dyn", |_, this, ()| {
            Ok(LuaMessageDescriptor(this.descriptor_dyn()))
        });

        methods.add_method("compute_size_dyn", |_, this, ()| {
            Ok(this.compute_size_dyn())
        });

        methods.add_method("is_instance_dyn", |_, this, ()| {
            Ok(this.is_initialized_dyn())
        });
    }
}