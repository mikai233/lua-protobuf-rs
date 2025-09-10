use derive_more::{Deref, From, Into};
use mlua::UserDataMethods;
use mlua::prelude::LuaUserData;
use protobuf::reflect::Syntax;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Deref, From, Into)]
pub struct LuaSyntax(pub Syntax);

impl LuaUserData for LuaSyntax {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("is_proto2", |_, this, ()| {
            Ok(matches!(this.0, Syntax::Proto2))
        });

        methods.add_method("is_proto3", |_, this, ()| {
            Ok(matches!(this.0, Syntax::Proto3))
        });
    }
}
