#[cfg(feature = "module")]
use mlua::Lua;
#[cfg(feature = "module")]
use mlua::prelude::{LuaAnyUserData, LuaResult};

#[cfg(feature = "module")]
use crate::pool::LuaProtoModule;

pub mod codec;
pub mod descriptor;
pub mod descriptor_proto;
mod dynamic_message;
mod message_dyn;
mod message_dyn_macros;
mod message_full_macros;
mod message_macros;
pub mod pool;
pub mod protoc;
mod reflect_value_box;
pub mod runtime_field_type;
pub mod runtime_type;
mod schema;
mod syntax;

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
