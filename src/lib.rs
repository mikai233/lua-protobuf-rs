use mlua::Lua;
use mlua::prelude::{LuaAnyUserData, LuaResult};

use crate::protoc::LuaProtoc;

pub mod codec;
pub mod descriptor;
pub mod descriptor_proto;
mod message_dyn;
mod message_dyn_macros;
mod message_full_macros;
mod message_macros;
pub mod protoc;
mod reflect_value_box;
pub mod runtime_field_type;
pub mod runtime_type;
mod syntax;

#[cfg(feature = "default")]
#[mlua::lua_module]
fn lua_protobuf_rs(lua: &Lua) -> LuaResult<LuaAnyUserData> {
    let protoc = lua.create_proxy::<LuaProtoc>()?;
    Ok(protoc)
}

#[cfg(feature = "default")]
#[mlua::lua_module]
fn liblua_protobuf_rs(lua: &Lua) -> LuaResult<LuaAnyUserData> {
    let protoc = lua.create_proxy::<LuaProtoc>()?;
    Ok(protoc)
}
