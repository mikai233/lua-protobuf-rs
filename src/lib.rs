use mlua::Lua;
use mlua::prelude::{LuaAnyUserData, LuaResult};

use crate::protoc::LuaProtoc;

pub mod codec;
pub mod protoc;
pub mod runtime_type;
pub mod runtime_field_type;
pub mod descriptor;
pub mod descriptor_proto;

#[mlua::lua_module]
fn lua_protobuf_rs(lua: &Lua) -> LuaResult<LuaAnyUserData> {
    let protoc = lua.create_proxy::<LuaProtoc>()?;
    Ok(protoc)
}