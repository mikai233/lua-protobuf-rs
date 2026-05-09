#[cfg(feature = "module")]
use mlua::Lua;
#[cfg(feature = "module")]
use mlua::prelude::{LuaAnyUserData, LuaResult};

#[cfg(feature = "module")]
use crate::pool::LuaProtoModule;

pub mod codec;
mod dynamic_message;
mod gen_lua;
pub mod pool;
mod schema;

#[cfg(feature = "module")]
#[mlua::lua_module]
fn lua_protobuf_rs(lua: &Lua) -> LuaResult<LuaAnyUserData> {
    let protoc = lua.create_proxy::<LuaProtoModule>()?;
    Ok(protoc)
}

#[cfg(feature = "module")]
#[mlua::lua_module]
fn liblua_protobuf_rs(lua: &Lua) -> LuaResult<LuaAnyUserData> {
    let protoc = lua.create_proxy::<LuaProtoModule>()?;
    Ok(protoc)
}
