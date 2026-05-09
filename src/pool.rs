use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, anyhow};
use mlua::prelude::LuaUserData;
use mlua::{Lua, String as LuaString, Table, UserDataMethods};
use protobuf::descriptor::{FileDescriptorProto, FileDescriptorSet};
use protobuf::reflect::{EnumDescriptor, FileDescriptor, MessageDescriptor, ServiceDescriptor};
use protobuf::{CodedInputStream, Message, MessageDyn};

use crate::codec::{CodecOptions, LuaProtoCodec};
use crate::dynamic_message::LuaDynamicMessage;
use crate::lua_annotations;
use crate::schema;

#[derive(Default)]
pub struct LuaProtoPool {
    codec: LuaProtoCodec,
    file_descriptors: HashMap<String, FileDescriptor>,
    message_descriptors: HashMap<String, MessageDescriptor>,
    enum_descriptors: HashMap<String, EnumDescriptor>,
    service_descriptors: HashMap<String, ServiceDescriptor>,
}

impl LuaProtoPool {
    pub fn new(descriptors: Vec<FileDescriptor>) -> Self {
        let codec = LuaProtoCodec;
        let mut pool = Self {
            codec,
            ..Self::default()
        };
        for file_descriptor in descriptors {
            pool.index_file(file_descriptor);
        }
        pool
    }

    pub fn parse_files(
        inputs: impl IntoIterator<Item = impl AsRef<Path>>,
        includes: impl IntoIterator<Item = impl AsRef<Path>>,
    ) -> anyhow::Result<Self> {
        let mut parser = protobuf_parse::Parser::new();
        parser.inputs(inputs).includes(includes);

        #[cfg(feature = "google_protoc")]
        parser.protoc();

        #[cfg(feature = "vendored_protoc")]
        parser.protoc_path(
            &protoc_bin_vendored::protoc_bin_path()
                .context("unable to find protoc bin vendored")?,
        );

        let file_protos = parser
            .parse_and_typecheck()
            .context("parse proto failed")?
            .file_descriptors;
        Self::from_file_protos(file_protos)
    }

    pub fn parse_proto(proto: impl AsRef<str>) -> anyhow::Result<Self> {
        let temp_dir = tempfile::tempdir().context("unable to get tempdir")?;
        let tempfile = temp_dir.path().join("temp.proto");
        std::fs::write(&tempfile, proto.as_ref()).context("unable to write data to tempfile")?;
        Self::parse_files([&tempfile], [&temp_dir])
    }

    pub fn parse_descriptor_path(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let path = path.as_ref();
        let protos = if path.is_dir() {
            let mut protos = vec![];
            for entry in walkdir::WalkDir::new(path)
                .into_iter()
                .filter_map(|file| file.ok())
            {
                let pb_path = entry.path();
                if pb_path.extension().map(|e| e == "pb").unwrap_or(false) {
                    let mut pb_file = std::fs::File::open(pb_path)
                        .context(format!("failed open {}", pb_path.to_string_lossy()))?;
                    let mut input = CodedInputStream::new(&mut pb_file);
                    let proto = FileDescriptorProto::parse_from(&mut input)?;
                    protos.push(proto);
                }
            }
            protos
        } else {
            let bytes = std::fs::read(path).context(format!(
                "failed read descriptor set {}",
                path.to_string_lossy()
            ))?;
            match FileDescriptorSet::parse_from_bytes(&bytes) {
                Ok(set) => set.file,
                Err(_) => vec![FileDescriptorProto::parse_from_bytes(&bytes)?],
            }
        };
        Self::from_file_protos(protos)
    }

    pub fn list_protos(paths: impl IntoIterator<Item = impl AsRef<Path>>) -> Vec<PathBuf> {
        let mut protos = Vec::new();
        for path in paths {
            for file in walkdir::WalkDir::new(path)
                .into_iter()
                .filter_map(|file| file.ok())
            {
                let proto_path = file.path();
                if proto_path
                    .extension()
                    .map(|e| e == "proto")
                    .unwrap_or(false)
                {
                    protos.push(proto_path.to_path_buf());
                }
            }
        }
        protos
    }

    pub fn descriptor_set_bytes(&self) -> anyhow::Result<Vec<u8>> {
        let mut set = FileDescriptorSet::default();
        let mut file_descriptors = self.file_descriptors.values().collect::<Vec<_>>();
        file_descriptors.sort_by_key(|descriptor| descriptor.name());
        set.file = file_descriptors
            .into_iter()
            .map(|descriptor| descriptor.proto().clone())
            .collect();
        set.write_to_bytes().map_err(|e| anyhow!(e))
    }

    pub fn write_descriptor_set(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).context(format!(
                "failed create descriptor set output directory {}",
                parent.to_string_lossy()
            ))?;
        }
        std::fs::write(path, self.descriptor_set_bytes()?).context(format!(
            "failed write descriptor set {}",
            path.to_string_lossy()
        ))
    }

    pub fn write_file_descriptors(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let path = path.as_ref();
        std::fs::create_dir_all(path).context(format!(
            "failed create file descriptor output directory {}",
            path.to_string_lossy()
        ))?;
        let mut file_descriptors = self.file_descriptors.values().collect::<Vec<_>>();
        file_descriptors.sort_by_key(|descriptor| descriptor.name());
        for file_descriptor in file_descriptors {
            let name = file_descriptor
                .name()
                .strip_suffix(".proto")
                .unwrap_or_else(|| file_descriptor.name());
            let file_path = path.join(format!("{name}.pb"));
            if let Some(parent) = file_path.parent() {
                std::fs::create_dir_all(parent).context(format!(
                    "failed create file descriptor output directory {}",
                    parent.to_string_lossy()
                ))?;
            }
            std::fs::write(&file_path, file_descriptor.proto().write_to_bytes()?).context(
                format!(
                    "failed write file descriptor {}",
                    file_path.to_string_lossy()
                ),
            )?;
        }
        Ok(())
    }

    pub fn encode(
        &self,
        message_full_name: &str,
        lua_message: &Table,
        options: CodecOptions,
    ) -> anyhow::Result<Box<dyn MessageDyn>> {
        let descriptor = self
            .message_descriptors
            .get(message_full_name)
            .ok_or(anyhow!("{} not found", message_full_name))?;
        self.codec.encode_message(lua_message, descriptor, options)
    }

    pub fn decode(
        &self,
        lua: &Lua,
        message_full_name: &str,
        message_bytes: &[u8],
        options: CodecOptions,
    ) -> anyhow::Result<Table> {
        let message = self.decode_box(message_full_name, message_bytes)?;
        self.codec.decode_message(lua, message.as_ref(), options)
    }

    fn decode_box(
        &self,
        message_full_name: &str,
        message_bytes: &[u8],
    ) -> anyhow::Result<Box<dyn MessageDyn>> {
        let descriptor = self
            .message_descriptors
            .get(message_full_name)
            .ok_or(anyhow!("{} not found", message_full_name))?;
        descriptor
            .parse_from_bytes(message_bytes)
            .map_err(|e| anyhow!(e))
    }

    fn from_file_protos(protos: Vec<FileDescriptorProto>) -> anyhow::Result<Self> {
        let file_descriptors = FileDescriptor::new_dynamic_fds(protos, &[])?;
        Ok(Self::new(file_descriptors))
    }

    fn index_file(&mut self, file_descriptor: FileDescriptor) {
        for message_descriptor in file_descriptor.messages() {
            self.index_message(message_descriptor);
        }
        for enum_descriptor in file_descriptor.enums() {
            self.enum_descriptors
                .insert(enum_descriptor.full_name().to_string(), enum_descriptor);
        }
        for service_descriptor in file_descriptor.services() {
            self.service_descriptors.insert(
                service_full_name(file_descriptor.package(), service_descriptor.proto().name()),
                service_descriptor,
            );
        }
        self.file_descriptors
            .insert(file_descriptor.name().to_string(), file_descriptor);
    }

    fn index_message(&mut self, message_descriptor: MessageDescriptor) {
        for nested_message in message_descriptor.nested_messages() {
            self.index_message(nested_message);
        }
        for nested_enum in message_descriptor.nested_enums() {
            self.enum_descriptors
                .insert(nested_enum.full_name().to_string(), nested_enum);
        }
        self.message_descriptors.insert(
            message_descriptor.full_name().to_string(),
            message_descriptor,
        );
    }

    fn codec_options(options: Option<Table>) -> mlua::Result<CodecOptions> {
        CodecOptions::from_lua_table(options).map_err(|e| anyhow!("{e:?}").into())
    }
}

impl LuaUserData for LuaProtoPool {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method(
            "encode",
            |lua, pool, (message_full_name, lua_message, options): (String, Table, Option<Table>)| {
                let options = Self::codec_options(options)?;
                let message = pool
                    .encode(&message_full_name, &lua_message, options)
                    .map_err(|e| anyhow!("{e:?}"))?;
                let mut message_bytes = Vec::with_capacity(message.compute_size_dyn() as usize);
                message
                    .write_to_vec_dyn(&mut message_bytes)
                    .map_err(|e| anyhow!("{e:?}"))?;
                lua.create_string(message_bytes)
            },
        );

        methods.add_method(
            "decode",
            |lua, pool, (message_full_name, message_bytes, options): (String, LuaString, Option<Table>)| {
                pool.decode(lua, &message_full_name, message_bytes.as_bytes().as_ref(), Self::codec_options(options)?)
                    .map_err(|e| anyhow!("{e:?}").into())
            },
        );

        methods.add_method(
            "validate",
            |_, pool, (message_full_name, lua_message, options): (String, Table, Option<Table>)| {
                let options = Self::codec_options(options)?;
                match pool.encode(&message_full_name, &lua_message, options) {
                    Ok(_) => Ok((true, None::<String>)),
                    Err(e) => Ok((false, Some(format!("{e:?}")))),
                }
            },
        );

        methods.add_method("new", |_, pool, message_full_name: String| {
            let descriptor = pool
                .message_descriptors
                .get(&message_full_name)
                .ok_or(anyhow!("{} not found", message_full_name))?;
            Ok(LuaDynamicMessage::new(descriptor.new_instance()))
        });

        methods.add_method(
            "decode_message",
            |_, pool, (message_full_name, message_bytes): (String, LuaString)| {
                let message = pool
                    .decode_box(&message_full_name, message_bytes.as_bytes().as_ref())
                    .map_err(|e| anyhow!("{e:?}"))?;
                Ok(LuaDynamicMessage::new(message))
            },
        );

        methods.add_method("files", |lua, pool, ()| {
            let files = lua.create_table()?;
            for descriptor in pool.file_descriptors.values() {
                files.push(schema::file_to_table(lua, descriptor)?)?;
            }
            Ok(files)
        });

        methods.add_method("file", |lua, pool, name: String| {
            match pool.file_descriptors.get(&name) {
                Some(descriptor) => Ok(Some(schema::file_to_table(lua, descriptor)?)),
                None => Ok(None),
            }
        });

        methods.add_method("messages", |lua, pool, ()| {
            let messages = lua.create_table()?;
            for descriptor in pool.message_descriptors.values() {
                messages.push(schema::message_to_table(lua, descriptor)?)?;
            }
            Ok(messages)
        });

        methods.add_method("message", |lua, pool, name: String| {
            match pool.message_descriptors.get(&name) {
                Some(descriptor) => Ok(Some(schema::message_to_table(lua, descriptor)?)),
                None => Ok(None),
            }
        });

        methods.add_method("enums", |lua, pool, ()| {
            let enums = lua.create_table()?;
            for descriptor in pool.enum_descriptors.values() {
                enums.push(schema::enum_to_table(lua, descriptor)?)?;
            }
            Ok(enums)
        });

        methods.add_method("enum", |lua, pool, name: String| {
            match pool.enum_descriptors.get(&name) {
                Some(descriptor) => Ok(Some(schema::enum_to_table(lua, descriptor)?)),
                None => Ok(None),
            }
        });

        methods.add_method("services", |lua, pool, ()| {
            let services = lua.create_table()?;
            for (full_name, descriptor) in &pool.service_descriptors {
                services.push(schema::service_to_table_with_full_name(
                    lua, descriptor, full_name,
                )?)?;
            }
            Ok(services)
        });

        methods.add_method("service", |lua, pool, name: String| {
            match pool.service_descriptors.get(&name) {
                Some(descriptor) => Ok(Some(schema::service_to_table_with_full_name(
                    lua, descriptor, &name,
                )?)),
                None => Ok(None),
            }
        });

        methods.add_method("gen_lua", |_, pool, path: String| {
            lua_annotations::generate(pool.file_descriptors.values(), path)
                .map_err(|e| anyhow!("{e:?}"))?;
            Ok(())
        });

        methods.add_method("descriptor_set", |lua, pool, ()| {
            let bytes = pool.descriptor_set_bytes().map_err(|e| anyhow!("{e:?}"))?;
            lua.create_string(bytes)
        });

        methods.add_method("write_descriptor_set", |_, pool, path: String| {
            pool.write_descriptor_set(path)
                .map_err(|e| anyhow!("{e:?}"))?;
            Ok(())
        });

        methods.add_method("write_file_descriptors", |_, pool, path: String| {
            pool.write_file_descriptors(path)
                .map_err(|e| anyhow!("{e:?}"))?;
            Ok(())
        });
    }
}

pub struct LuaProtoModule;

impl LuaUserData for LuaProtoModule {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("load", |_, config: Table| {
            if let Some(proto) = config.get::<Option<String>>("proto")? {
                return LuaProtoPool::parse_proto(proto).map_err(|e| anyhow!("{e:?}").into());
            }
            if let Some(path) = config.get::<Option<String>>("descriptor_set")? {
                return LuaProtoPool::parse_descriptor_path(path)
                    .map_err(|e| anyhow!("{e:?}").into());
            }
            let files = config.get::<Option<Vec<String>>>("files")?.ok_or(anyhow!(
                "load config requires files, proto, or descriptor_set"
            ))?;
            let includes = config
                .get::<Option<Vec<String>>>("includes")?
                .unwrap_or_else(|| vec![".".to_string()]);
            LuaProtoPool::parse_files(files, includes).map_err(|e| anyhow!("{e:?}").into())
        });

        methods.add_function(
            "load_files",
            |_, (inputs, includes): (Vec<String>, Vec<String>)| {
                if inputs.is_empty() {
                    return Err(anyhow!("inputs must not be empty").into());
                }
                if includes.is_empty() {
                    return Err(anyhow!("includes must not be empty").into());
                }
                LuaProtoPool::parse_files(inputs, includes).map_err(|e| anyhow!("{e:?}").into())
            },
        );

        methods.add_function("load_proto", |_, proto: String| {
            LuaProtoPool::parse_proto(proto).map_err(|e| anyhow!("{e:?}").into())
        });

        methods.add_function("load_descriptor_set", |_, path: String| {
            LuaProtoPool::parse_descriptor_path(path).map_err(|e| anyhow!("{e:?}").into())
        });

        methods.add_function("list_protos", |_, paths: Vec<String>| {
            let protos = LuaProtoPool::list_protos(paths)
                .iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect::<Vec<String>>();
            Ok(protos)
        });
    }
}

fn service_full_name(package: &str, name: &str) -> String {
    if package.is_empty() {
        name.to_string()
    } else {
        format!("{package}.{name}")
    }
}

#[cfg(all(test, not(feature = "module")))]
mod tests {
    use mlua::Lua;

    use super::*;
    use crate::codec::CodecOptions;

    #[test]
    fn pool_loads_schema_and_roundtrips_dynamic_messages() -> anyhow::Result<()> {
        let lua = Lua::new();
        let pool = LuaProtoPool::parse_proto(
            r#"
            syntax = "proto3";
            package demo;

            enum State {
              UNKNOWN = 0;
              ONLINE = 1;
            }

            message Player {
              int64 id = 1;
              string name = 2;
              State state = 3;
              bytes payload = 4;
            }
            "#,
        )?;

        let descriptor = pool
            .message_descriptors
            .get("demo.Player")
            .expect("message descriptor");
        let schema = schema::message_to_table(&lua, descriptor)?;
        assert_eq!(schema.get::<String>("full_name")?, "demo.Player");

        let input = lua.create_table()?;
        input.set("id", "9223372036854775807")?;
        input.set("name", "mikai233")?;
        input.set("state", "ONLINE")?;
        input.set("payload", lua.create_string([1_u8, 2, 3])?)?;

        let message = pool.encode("demo.Player", &input, CodecOptions::default())?;
        let mut bytes = Vec::new();
        message.write_to_vec_dyn(&mut bytes)?;

        let output = pool.decode(&lua, "demo.Player", &bytes, CodecOptions::default())?;
        assert_eq!(
            output.get::<String>("id")?,
            "9223372036854775807".to_string()
        );
        assert_eq!(output.get::<String>("name")?, "mikai233".to_string());
        assert_eq!(output.get::<String>("state")?, "ONLINE".to_string());
        assert_eq!(
            output.get::<mlua::String>("payload")?.as_bytes().as_ref(),
            &[1, 2, 3]
        );

        let dir = tempfile::tempdir()?;
        crate::lua_annotations::generate(pool.file_descriptors.values(), dir.path())?;
        let generated = std::fs::read_to_string(dir.path().join("temp.lua"))?;
        assert!(generated.contains("---@class demo_Player"));
        assert!(generated.contains("---@field id? string"));
        assert!(generated.contains("---@field payload? string"));
        assert!(generated.contains("---@class demo_State"));

        Ok(())
    }

    #[test]
    fn lua_module_proxy_exercises_public_pool_api() -> anyhow::Result<()> {
        let lua = Lua::new();
        let module = lua.create_proxy::<LuaProtoModule>()?;
        lua.globals().set("pb", module)?;
        let descriptor_dir = tempfile::tempdir()?;
        let descriptor_set_path = descriptor_dir.path().join("demo.pb");
        let file_descriptor_dir = descriptor_dir.path().join("fds");
        lua.globals().set(
            "descriptor_set_path",
            descriptor_set_path.to_string_lossy().to_string(),
        )?;
        lua.globals().set(
            "file_descriptor_dir",
            file_descriptor_dir.to_string_lossy().to_string(),
        )?;

        lua.load(
            r#"
            local proto = [[
            syntax = "proto3";
            package demo;

            enum State {
              UNKNOWN = 0;
              ONLINE = 1;
            }

            message Player {
              int64 id = 1;
              string name = 2;
              State state = 3;
              bytes payload = 4;
              repeated int32 scores = 5;
              map<string, int64> attrs = 6;
              oneof contact {
                string email = 7;
                string phone = 8;
              }
            }

            service PlayerService {
              rpc Get(Player) returns (Player);
            }
            ]]

            local pool = pb.load_proto(proto)
            local bytes = pool:encode("demo.Player", {
                id = "9223372036854775807",
                name = "mikai233",
                state = "ONLINE",
                payload = "\1\2\3",
                scores = { 7, 8 },
                attrs = { hp = "99" },
                email = "dev@example.com",
            })

            assert(type(bytes) == "string")

            local player = pool:decode("demo.Player", bytes)
            assert(player.id == "9223372036854775807")
            assert(player.name == "mikai233")
            assert(player.state == "ONLINE")
            assert(player.payload == "\1\2\3")
            assert(player.scores[1] == 7)
            assert(player.attrs.hp == "99")
            assert(player.email == "dev@example.com")

            local desc = pool:message("demo.Player")
            assert(desc.full_name == "demo.Player")
            assert(#desc.fields == 8)
            assert(desc.fields[1].name == "id")
            assert(desc.fields[1].type == "int64")
            assert(desc.file == "temp.proto")
            assert(desc.package == "demo")
            assert(desc.fields_by_name.id.number == 1)
            assert(desc.fields_by_json_name.id.name == "id")
            assert(desc.fields_by_number[1].name == "id")
            assert(desc.fields_by_name.attrs.resolved_type.kind == "map")
            assert(desc.fields_by_name.attrs.resolved_type.key.kind == "string")
            assert(desc.fields_by_name.attrs.resolved_type.value.kind == "int64")
            assert(desc.fields_by_name.state.resolved_type.kind == "enum")
            assert(desc.fields_by_name.state.resolved_type.full_name == "demo.State")
            assert(desc.fields_by_name.email.oneof == "contact")
            assert(desc.oneofs_by_name.contact.field_names[1] == "email")
            assert(desc.oneofs_by_name.contact.fields_by_name.phone.number == 8)

            local state = pool:enum("demo.State")
            assert(state.default_value.name == "UNKNOWN")
            assert(state.values_by_name.ONLINE.number == 1)
            assert(state.values_by_number[1].name == "ONLINE")
            assert(state.values[2].name == "ONLINE")
            assert(state.values[2].number == 1)

            local service = pool:service("demo.PlayerService")
            assert(service.full_name == "demo.PlayerService")
            assert(service.methods_by_name.Get.input_type == "demo.Player")
            assert(service.methods_by_name.Get.output.full_name == "demo.Player")

            local file = pool:file("temp.proto")
            assert(file.messages_by_name["demo.Player"].name == "Player")
            assert(file.enums_by_name["demo.State"].name == "State")
            assert(file.services_by_name["demo.PlayerService"].name == "PlayerService")

            local descriptor_bytes = pool:descriptor_set()
            assert(type(descriptor_bytes) == "string")
            assert(#descriptor_bytes > 0)

            pool:write_descriptor_set(descriptor_set_path)
            local from_set = pb.load_descriptor_set(descriptor_set_path)
            assert(from_set:message("demo.Player").full_name == "demo.Player")
            assert(from_set:service("demo.PlayerService").methods_by_name.Get.input_type == "demo.Player")

            pool:write_file_descriptors(file_descriptor_dir)
            local from_files = pb.load_descriptor_set(file_descriptor_dir)
            assert(from_files:enum("demo.State").values_by_name.ONLINE.number == 1)
            "#,
        )
        .exec()?;

        Ok(())
    }

    #[test]
    fn lua_codec_options_are_stable() -> anyhow::Result<()> {
        let lua = Lua::new();
        let module = lua.create_proxy::<LuaProtoModule>()?;
        lua.globals().set("pb", module)?;

        lua.load(
            r#"
            local proto = [[
            syntax = "proto3";
            package demo;

            enum State {
              UNKNOWN = 0;
              ONLINE = 1;
            }

            message Player {
              int64 id = 1;
              string name = 2;
              State state = 3;
              bytes payload = 4;
              repeated int32 scores = 5;
              map<string, int64> attrs = 6;
              oneof contact {
                string email = 7;
                string phone = 8;
              }
            }
            ]]

            local pool = pb.load({ proto = proto })

            local empty_bytes = pool:encode("demo.Player", {})
            local sparse = pool:decode("demo.Player", empty_bytes)
            assert(sparse.id == nil)
            assert(sparse.name == nil)
            assert(sparse.scores == nil)
            assert(sparse.attrs == nil)
            assert(sparse.email == nil)

            local with_defaults = pool:decode("demo.Player", empty_bytes, {
                defaults = true,
                enum = "number",
                int64 = "integer",
            })
            assert(with_defaults.id == 0)
            assert(with_defaults.name == "")
            assert(with_defaults.state == 0)
            assert(type(with_defaults.scores) == "table" and #with_defaults.scores == 0)
            assert(type(with_defaults.attrs) == "table")
            assert(with_defaults.email == nil)

            local table_bytes = pool:encode("demo.Player", {
                id = 42,
                state = 1,
                payload = { 9, 8, 7 },
                attrs = { hp = 100 },
            }, {
                bytes = "table",
                enum = "number",
                int64 = "integer",
            })
            local table_decoded = pool:decode("demo.Player", table_bytes, {
                bytes = "table",
                enum = "number",
                int64 = "integer",
            })
            assert(table_decoded.id == 42)
            assert(table_decoded.state == 1)
            assert(table_decoded.payload[1] == 9)
            assert(table_decoded.payload[3] == 7)
            assert(table_decoded.attrs.hp == 100)

            local ok = pcall(function()
                pool:encode("demo.Player", { unknown_field = 1 })
            end)
            assert(ok == false)

            local valid, validation_error = pool:validate("demo.Player", {
                attrs = { hp = {} },
            })
            assert(valid == false, tostring(validation_error))
            assert(validation_error:find('demo.Player.attrs["hp"]', 1, true) ~= nil, validation_error)
            assert(validation_error:find("int64", 1, true) ~= nil, validation_error)

            local valid_unknown, unknown_error = pool:validate("demo.Player", {
                unknown_field = 1,
            })
            assert(valid_unknown == false, tostring(unknown_error))
            assert(unknown_error:find("demo.Player.unknown_field", 1, true) ~= nil, unknown_error)

            local valid_ok, no_error = pool:validate("demo.Player", {
                id = "42",
                attrs = { hp = "100" },
            })
            assert(valid_ok == true)
            assert(no_error == nil)

            local ignored = pool:decode("demo.Player", pool:encode("demo.Player", {
                unknown_field = 1,
                name = "kept",
            }, {
                unknown = "ignore",
            }))
            assert(ignored.name == "kept")
            assert(ignored.unknown_field == nil)
            "#,
        )
        .exec()?;

        Ok(())
    }

    #[test]
    fn lua_dynamic_message_api_roundtrips_fields() -> anyhow::Result<()> {
        let lua = Lua::new();
        let module = lua.create_proxy::<LuaProtoModule>()?;
        lua.globals().set("pb", module)?;

        lua.load(
            r#"
            local proto = [[
            syntax = "proto3";
            package demo;

            message Player {
              int64 id = 1;
              string name = 2;
              repeated int32 scores = 3;
              map<string, int64> attrs = 4;
            }
            ]]

            local pool = pb.load_proto(proto)
            local msg = pool:new("demo.Player")
            assert(msg:type_name() == "demo.Player")
            assert(msg:has("name") == false)

            msg:set("id", "123")
            msg:set("name", "mikai233")
            msg:set("scores", { 1, 2, 3 })
            msg:set("attrs", { hp = "99" })

            assert(msg:has("name") == true)
            assert(msg:get("id") == "123")
            assert(msg:get("scores")[3] == 3)
            assert(msg:get("attrs").hp == "99")

            local table_value = msg:to_table()
            assert(table_value.name == "mikai233")
            assert(table_value.attrs.hp == "99")

            msg:clear("name")
            assert(msg:has("name") == false)
            assert(msg:get("name") == nil)

            local bytes = msg:encode()
            local decoded = pool:decode("demo.Player", bytes)
            assert(decoded.id == "123")
            assert(decoded.name == nil)
            assert(decoded.scores[2] == 2)
            "#,
        )
        .exec()?;

        Ok(())
    }
}
