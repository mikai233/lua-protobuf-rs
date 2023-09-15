use std::collections::HashMap;
use std::path::{Path, PathBuf};
use anyhow::{anyhow, Context};
use mlua::prelude::LuaUserData;
use mlua::{ExternalError, Lua, Table, UserDataMethods};
use protobuf::MessageDyn;
use protobuf::reflect::FileDescriptor;
use crate::codec::LuaProtoCodec;
use crate::descriptor::enum_descriptor::LuaEnumDescriptor;
use crate::descriptor::file_descriptor::LuaFileDescriptor;
use crate::descriptor::message_descriptor::LuaMessageDescriptor;

#[derive(Default)]
pub struct LuaProtoc {
    pub codec: LuaProtoCodec,
    pub file_descriptors: HashMap<String, LuaFileDescriptor>,
    pub message_descriptors: HashMap<String, LuaMessageDescriptor>,
    pub enum_descriptors: HashMap<String, LuaEnumDescriptor>,
}

impl LuaProtoc {
    pub fn compile_file(inputs: impl IntoIterator<Item=impl AsRef<Path>>, includes: impl IntoIterator<Item=impl AsRef<Path>>) -> anyhow::Result<Self> {
        let protoc_path = protoc_bin_vendored::protoc_bin_path().context("unable to find protoc bin vendored")?;
        let file_protos = protobuf_parse::Parser::new()
            .protoc()
            .protoc_path(&*protoc_path)
            .inputs(inputs)
            .includes(includes)
            .parse_and_typecheck()?
            .file_descriptors;
        let mut file_descriptors = HashMap::new();
        let mut message_descriptors = HashMap::new();
        let mut enum_descriptors = HashMap::new();
        for file_descriptor in FileDescriptor::new_dynamic_fds(file_protos, &[])? {
            for message_descriptor in file_descriptor.messages() {
                message_descriptors.insert(message_descriptor.full_name().to_string(), From::from(message_descriptor));
            }
            for enum_descriptor in file_descriptor.enums() {
                enum_descriptors.insert(enum_descriptor.full_name().to_string(), From::from(enum_descriptor));
            }
            file_descriptors.insert(file_descriptor.name().to_string(), From::from(file_descriptor));
        };
        let protoc = LuaProtoc {
            codec: Default::default(),
            file_descriptors,
            message_descriptors,
            enum_descriptors,
        };
        Ok(protoc)
    }

    pub fn compile_proto(proto: String) -> anyhow::Result<Self> {
        let temp_dir = tempfile::tempdir().context("unable to get tempdir")?;
        let tempfile = temp_dir.path().join("temp.proto");
        std::fs::write(&tempfile, proto).context("unable to write data to tempfile")?;
        LuaProtoc::compile_file([&tempfile], [&temp_dir])
    }

    pub fn encode(&self, message_full_name: String, lua_message: Table) -> anyhow::Result<Box<dyn MessageDyn>> {
        let descriptor = self.message_descriptors.get(&message_full_name).ok_or(anyhow!("{} not found",message_full_name))?;
        let message = self.codec.encode_message(lua_message, descriptor)?;
        Ok(message)
    }

    pub fn decode<'a>(&self, lua: &'a Lua, message_full_name: String, message_bytes: &[u8]) -> anyhow::Result<Table<'a>> {
        let descriptor = self.message_descriptors.get(&message_full_name).ok_or(anyhow!("{} not found",message_full_name))?;
        let message = descriptor.parse_from_bytes(message_bytes)?;
        let lua_message = self.codec.decode_message(message.as_ref(), lua)?;
        Ok(lua_message)
    }

    pub fn list_protos(dirs: impl IntoIterator<Item=impl AsRef<Path>>) -> Vec<PathBuf> {
        let mut protos = Vec::new();
        for dir in dirs {
            for file in walkdir::WalkDir::new(dir).into_iter().filter_map(|file| file.ok()) {
                let proto_path = file.path();
                if proto_path.extension().and_then(|e| Some(e == "proto")).unwrap_or(false) {
                    protos.push(proto_path.to_path_buf());
                }
            }
        }
        protos
    }
}

impl LuaUserData for LuaProtoc {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function("compile_file", |_, (inputs, includes): (Vec<String>, Vec<String>)| {
            if includes.is_empty() {
                return Err(anyhow!("includes mut not empty").into_lua_err());
            }
            let protoc = LuaProtoc::compile_file(inputs, includes).map_err(|e| e.into_lua_err())?;
            Ok(protoc)
        });
        methods.add_function("compile_proto", |_, proto: String| {
            let protoc = LuaProtoc::compile_proto(proto).map_err(|e| e.into_lua_err())?;
            Ok(protoc)
        });
        methods.add_method("encode", |_, protoc, (message_full_name, lua_message): (String, Table)| {
            let message = protoc.encode(message_full_name, lua_message).map_err(|e| e.into_lua_err())?;
            let message_bytes = message.write_to_bytes_dyn().map_err(|e| e.into_lua_err())?;
            Ok(message_bytes)
        });
        methods.add_method("decode", |lua, protoc, (message_full_name, message_bytes): (String, Vec<u8>)| {
            let message = protoc.decode(lua, message_full_name, message_bytes.as_ref()).map_err(|e| e.into_lua_err())?;
            Ok(message)
        });
        methods.add_function("list_protos", |_, dirs: Vec<String>| {
            let protos = LuaProtoc::list_protos(dirs).iter().map(|p| { p.to_string_lossy().to_string() }).collect::<Vec<String>>();
            Ok(protos)
        });
        methods.add_method("all_file_descriptors", |_, protoc, ()| {
            let descriptors: Vec<_> = protoc.file_descriptors.values().map(Clone::clone).collect();
            Ok(descriptors)
        });
        methods.add_method("file_descriptor_by_name", |_, protoc, name: String| {
            let descriptor = protoc.file_descriptors.get(&name).map(Clone::clone);
            Ok(descriptor)
        });
        methods.add_method("all_message_descriptors", |_, protoc, ()| {
            let descriptors: Vec<_> = protoc.message_descriptors.values().map(Clone::clone).collect();
            Ok(descriptors)
        });
        methods.add_method("message_descriptor_by_name", |_, protoc, name: String| {
            let descriptor = protoc.message_descriptors.get(&name).map(Clone::clone);
            Ok(descriptor)
        });
        methods.add_method("all_enum_descriptors", |_, protoc, ()| {
            let descriptors: Vec<_> = protoc.enum_descriptors.values().map(Clone::clone).collect();
            Ok(descriptors)
        });
        methods.add_method("enum_descriptor_by_name", |_, protoc, name: String| {
            let descriptor = protoc.enum_descriptors.get(&name).map(Clone::clone);
            Ok(descriptor)
        });
    }
}