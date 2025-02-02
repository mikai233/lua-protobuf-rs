use mlua::prelude::{LuaAnyUserData, LuaResult};
use mlua::Lua;

use crate::protoc::LuaProtoc;

pub mod codec;
pub mod protoc;
pub mod runtime_type;
pub mod runtime_field_type;
pub mod descriptor;
pub mod descriptor_proto;
mod message_dyn;
mod reflect_value_box;
mod syntax;
mod message_macros;
mod message_dyn_macros;
mod message_full_macros;

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