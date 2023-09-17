use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context};
use mlua::{ExternalError, Lua, Table, UserDataMethods};
use mlua::prelude::LuaUserData;
use protobuf::{CodedInputStream, Message, MessageDyn};
use protobuf::descriptor::FileDescriptorProto;
use protobuf::reflect::{EnumDescriptor, FileDescriptor, MessageDescriptor, RuntimeFieldType, RuntimeType};

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
    pub fn new(descriptors: Vec<FileDescriptor>) -> Self {
        let codec = LuaProtoCodec::default();
        let mut file_descriptors = HashMap::new();
        let mut message_descriptors = HashMap::new();
        let mut enum_descriptors = HashMap::new();
        for file_descriptor in descriptors {
            for message_descriptor in file_descriptor.messages() {
                message_descriptors.insert(message_descriptor.full_name().to_string(), From::from(message_descriptor));
            }
            for enum_descriptor in file_descriptor.enums() {
                enum_descriptors.insert(enum_descriptor.full_name().to_string(), From::from(enum_descriptor));
            }
            file_descriptors.insert(file_descriptor.name().to_string(), file_descriptor.into());
        };
        Self {
            codec,
            file_descriptors,
            message_descriptors,
            enum_descriptors,
        }
    }

    pub fn compile_file(inputs: impl IntoIterator<Item=impl AsRef<Path>>, includes: impl IntoIterator<Item=impl AsRef<Path>>) -> anyhow::Result<Self> {
        let mut parser = protobuf_parse::Parser::new();
        parser.inputs(inputs).includes(includes);

        #[cfg(feature = "google_protoc")]
        parser.protoc();

        #[cfg(feature = "vendored_protoc")]
        parser.protoc_path(&protoc_bin_vendored::protoc_bin_path().context("unable to find protoc bin vendored")?);

        let file_protos = parser.parse_and_typecheck()?.file_descriptors;
        let file_descriptors: Vec<FileDescriptor> = FileDescriptor::new_dynamic_fds(file_protos, &[])?;
        let protoc = LuaProtoc::new(file_descriptors);
        Ok(protoc)
    }

    pub fn compile_proto(proto: String) -> anyhow::Result<Self> {
        let temp_dir = tempfile::tempdir().context("unable to get tempdir")?;
        let tempfile = temp_dir.path().join("temp.proto");
        std::fs::write(&tempfile, proto).context("unable to write data to tempfile")?;
        LuaProtoc::compile_file([&tempfile], [&temp_dir])
    }

    pub fn parse_pb(dir: String) -> anyhow::Result<Self> {
        let mut protos = vec![];
        for entry in walkdir::WalkDir::new(dir).into_iter().filter_map(|file| file.ok()) {
            let pb_path = entry.path();
            if pb_path.extension().and_then(|e| Some(e == "pb")).unwrap_or(false) {
                let mut pb_file = std::fs::File::open(pb_path).context(format!("failed open {}", pb_path.to_string_lossy()))?;
                let mut input = CodedInputStream::new(&mut pb_file);
                let proto = FileDescriptorProto::parse_from(&mut input)?;
                protos.push(proto);
            }
        }
        let file_descriptors = FileDescriptor::new_dynamic_fds(protos, &[])?;
        let protoc = LuaProtoc::new(file_descriptors);
        Ok(protoc)
    }

    pub fn gen_pb(&self, dir: String) -> anyhow::Result<()> {
        let path = PathBuf::from(dir);
        for (_, file_descriptor) in &self.file_descriptors {
            let name = file_descriptor.name().strip_suffix(".proto").expect("file descriptor not a proto file");
            let file_name = format!("{}.pb", name);
            let file_path = path.join(file_name);
            std::fs::write(&file_path, file_descriptor.proto().write_to_bytes()?).context(format!("failed write lua to file {}", file_path.to_string_lossy()))?;
        }
        Ok(())
    }

    pub fn gen_lua(&self, dir: String) -> anyhow::Result<()> {
        let path = PathBuf::from(dir);
        for (_, file_descriptor) in &self.file_descriptors {
            let name = file_descriptor.name().strip_suffix(".proto").expect("file descriptor not a proto file");
            let mut message_or_enum = vec![];
            for message_descriptor in file_descriptor.messages() {
                let name = message_descriptor.name().to_string();
                let mut nested_messages_or_enums = vec![];
                let nested_messages = message_descriptor.nested_messages().map(|m| (m.name().to_string(), name.clone()));
                nested_messages_or_enums.extend(nested_messages);
                let nested_enums = message_descriptor.nested_enums().map(|e| (e.name().to_string(), name.clone()));
                nested_messages_or_enums.extend(nested_enums);
                let messages = self.gen_lua_message(None, nested_messages_or_enums, &message_descriptor);
                message_or_enum.extend(messages);
            }
            for enum_descriptor in file_descriptor.enums() {
                let enum_table = self.gen_lua_enum(None, &enum_descriptor);
                message_or_enum.push(enum_table);
            }
            let code = message_or_enum.join("\n");
            let file_name = format!("{}.lua", name);
            let file_path = path.join(file_name);
            std::fs::write(&file_path, code).context(format!("failed write lua to file {}", file_path.to_string_lossy()))?;
        }
        Ok(())
    }

    fn gen_lua_message(&self, parent: Option<String>, nested_messages_or_enums: Vec<(String, String)>, descriptor: &MessageDescriptor) -> Vec<String> {
        if descriptor.is_map_entry() {
            return vec![];
        }
        let mut messages = vec![];
        let message_name = descriptor.name().to_string();
        let message_with_parent = self.decorate_with_parent(&parent, message_name.clone());
        let class = format!("---@class {}", message_with_parent);
        for nested_message_descriptor in descriptor.nested_messages() {
            let name = self.decorate_with_parent(&Some(message_with_parent.clone()), nested_message_descriptor.name().to_string());
            let mut child_nested_messages_or_enums = nested_messages_or_enums.clone();
            child_nested_messages_or_enums.extend(nested_message_descriptor.nested_messages().map(|m| (m.name().to_string(), name.clone())));
            child_nested_messages_or_enums.extend(nested_message_descriptor.nested_enums().map(|e| (e.name().to_string(), name.clone())));
            let nested_messages = self.gen_lua_message(Some(message_with_parent.clone()), child_nested_messages_or_enums, &nested_message_descriptor);
            messages.extend(nested_messages);
        }
        for nested_enum_descriptor in descriptor.nested_enums() {
            let nested_enum = self.gen_lua_enum(Some(message_with_parent.clone()), &nested_enum_descriptor);
            messages.push(nested_enum);
        }
        let mut fields = vec![];
        for field in descriptor.fields() {
            let parent = self.decorate_message_type_with_parent(field.runtime_field_type(), &nested_messages_or_enums);
            let ty = self.lua_type_of(parent.clone(), field.runtime_field_type());
            let field = format!("---@field {} {}", field.name(), ty);
            fields.push(field)
        }
        let message_table = if fields.is_empty() {
            format!("{}\nlocal {} = {{ }}\n", class, message_with_parent)
        } else {
            format!("{}\n{}\nlocal {} = {{ }}\n", class, fields.join("\n"), message_with_parent)
        };
        messages.push(message_table);
        messages
    }

    fn gen_lua_enum(&self, parent: Option<String>, descriptor: &EnumDescriptor) -> String {
        let name = descriptor.name();
        let message = match parent {
            None => {
                format!("{}", name)
            }
            Some(parent) => {
                format!("{}_{}", parent, name)
            }
        };
        let class = format!("---@class {}", message);
        let mut fields = vec![];
        let mut enum_kv = vec![];
        for value in descriptor.values() {
            let field = format!("---@field {} number {}", value.name(), value.value());
            fields.push(field);
            enum_kv.push((value.name().to_string(), value.value().to_string()))
        }
        let kvs = enum_kv.iter().map(|(k, v)| format!("{} = {}", k, v)).collect::<Vec<String>>();
        format!("{}\n{}\n{} = {{ {} }}\n", class, fields.join("\n"), message, kvs.join(", "))
    }

    fn decorate_with_parent(&self, parent: &Option<String>, name: String) -> String {
        let message = match &parent {
            None => {
                format!("{}", name)
            }
            Some(parent) => {
                format!("{}_{}", parent, name)
            }
        };
        message
    }

    fn decorate_message_type_with_parent(&self, runtime_field_type: RuntimeFieldType, nested_messages_or_enums: &Vec<(String, String)>) -> Option<String> {
        fn find_message(nested_messages_or_enums: &Vec<(String, String)>, name: &str) -> Option<String> {
            match nested_messages_or_enums.iter().rfind(|(n, _)| n == name) {
                None => {
                    None
                }
                Some((_, p)) => {
                    Some(p.clone())
                }
            }
        }

        fn decorate_message(nested_messages_or_enums: &Vec<(String, String)>, rt: RuntimeType) -> Option<String> {
            match rt {
                RuntimeType::Enum(e) => {
                    find_message(nested_messages_or_enums, e.name())
                }
                RuntimeType::Message(m) => {
                    find_message(nested_messages_or_enums, m.name())
                }
                _ => None
            }
        }
        match runtime_field_type {
            RuntimeFieldType::Singular(rt) => {
                decorate_message(nested_messages_or_enums, rt)
            }
            RuntimeFieldType::Repeated(rt) => {
                decorate_message(nested_messages_or_enums, rt)
            }
            RuntimeFieldType::Map(_, value_rt) => {
                decorate_message(nested_messages_or_enums, value_rt)
            }
        }
    }

    fn lua_type_of(&self, parent: Option<String>, field_type: RuntimeFieldType) -> String {
        fn type_of(protoc: &LuaProtoc, parent: Option<String>, rt: RuntimeType) -> String {
            let ty = match rt {
                RuntimeType::I32 |
                RuntimeType::I64 |
                RuntimeType::U32 |
                RuntimeType::U64 |
                RuntimeType::F32 |
                RuntimeType::F64 => "number".to_string(),
                RuntimeType::Bool => "boolean".to_string(),
                RuntimeType::String => "string".to_string(),
                RuntimeType::VecU8 => "number[]".to_string(),
                RuntimeType::Enum(e) => {
                    let name = e.name().to_string();
                    protoc.decorate_with_parent(&parent, name)
                }
                RuntimeType::Message(m) => {
                    let name = m.name().to_string();
                    protoc.decorate_with_parent(&parent, name)
                }
            };
            ty
        }
        let ty = match field_type {
            RuntimeFieldType::Singular(rt) => {
                type_of(self, parent, rt)
            }
            RuntimeFieldType::Repeated(rt) => {
                format!("{}[]", type_of(self, parent, rt))
            }
            RuntimeFieldType::Map(key_rt, value_rt) => {
                format!("table<{},{}>", type_of(self, parent.clone(), key_rt), type_of(self, parent, value_rt))
            }
        };
        ty
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
            if inputs.is_empty() {
                return Err(anyhow!("inputs mut not empty").into_lua_err());
            }
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
        methods.add_function("parse_pb", |_, dir: String| {
            let protoc = LuaProtoc::parse_pb(dir).map_err(|e| e.into_lua_err())?;
            Ok(protoc)
        });
        methods.add_method("gen_pb", |_, this, dir: String| {
            this.gen_pb(dir).map_err(|e| e.into_lua_err())?;
            Ok(())
        });
        methods.add_method("gen_lua", |_, this, dir: String| {
            this.gen_lua(dir).map_err(|e| e.into_lua_err())?;
            Ok(())
        });
        methods.add_method("encode", |_, protoc, (message_full_name, lua_message): (String, Table)| {
            let message = protoc.encode(message_full_name, lua_message).map_err(|e| e.into_lua_err())?;
            let mut message_bytes = Vec::with_capacity(message.compute_size_dyn() as usize);
            message.write_to_vec_dyn(&mut message_bytes).map_err(|e| e.into_lua_err())?;
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