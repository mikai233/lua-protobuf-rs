use mlua::Lua;
use mlua::prelude::{LuaAnyUserData, LuaResult};

use crate::protoc::LuaProtoc;

pub mod codec;
pub mod protoc;
pub mod descriptor;
pub mod field_descriptor_proto;
pub mod runtime_type;
pub mod runtime_field_type;

#[mlua::lua_module]
fn lua_protobuf_rs(lua: &Lua) -> LuaResult<LuaAnyUserData> {
    let protoc = lua.create_proxy::<LuaProtoc>()?;
    Ok(protoc)
}