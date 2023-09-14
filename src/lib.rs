use mlua::Lua;
use mlua::prelude::{LuaAnyUserData, LuaResult};

use crate::protoc::LuaProtoc;

mod codec;
mod protoc;

#[mlua::lua_module]
fn lua_protobuf_rs(lua: &Lua) -> LuaResult<LuaAnyUserData> {
    let protoc = lua.create_proxy::<LuaProtoc>()?;
    Ok(protoc)
}