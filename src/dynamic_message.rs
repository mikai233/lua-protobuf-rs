use anyhow::anyhow;
use mlua::prelude::LuaUserData;
use mlua::{Table, UserDataMethods, Value};
use protobuf::MessageDyn;
use protobuf::reflect::RuntimeFieldType;

use crate::codec::{CodecOptions, LuaProtoCodec, UnknownFieldMode};
use crate::schema;

pub struct LuaDynamicMessage {
    message: Box<dyn MessageDyn>,
}

impl LuaDynamicMessage {
    pub fn new(message: Box<dyn MessageDyn>) -> Self {
        Self { message }
    }

    fn codec_options(options: Option<Table>) -> mlua::Result<CodecOptions> {
        CodecOptions::from_lua_table(options).map_err(|e| anyhow!("{e:?}").into())
    }
}

impl LuaUserData for LuaDynamicMessage {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("type_name", |_, this, ()| {
            Ok(this.message.descriptor_dyn().full_name().to_string())
        });

        methods.add_method("descriptor", |lua, this, ()| {
            schema::message_to_table(lua, &this.message.descriptor_dyn())
        });

        methods.add_method("has", |_, this, field_name: String| {
            let descriptor = this.message.descriptor_dyn();
            let field = descriptor
                .field_by_name_or_json_name(&field_name)
                .ok_or(anyhow!(
                    "field {} not found in {}",
                    field_name,
                    descriptor.full_name()
                ))?;
            Ok(field.has_field(this.message.as_ref()))
        });

        methods.add_method("which_oneof", |_, this, oneof_name: String| {
            let descriptor = this.message.descriptor_dyn();
            let oneof = descriptor
                .oneofs()
                .find(|oneof| oneof.name() == oneof_name)
                .ok_or(anyhow!(
                    "oneof {} not found in {}",
                    oneof_name,
                    descriptor.full_name()
                ))?;
            for field in oneof.fields() {
                if field.has_field(this.message.as_ref()) {
                    return Ok(Some(field.name().to_string()));
                }
            }
            Ok(None::<String>)
        });

        methods.add_method(
            "get",
            |lua, this, (field_name, options): (String, Option<Table>)| {
                let options = Self::codec_options(options)?;
                let descriptor = this.message.descriptor_dyn();
                let field = descriptor
                    .field_by_name_or_json_name(&field_name)
                    .ok_or(anyhow!(
                        "field {} not found in {}",
                        field_name,
                        descriptor.full_name()
                    ))?;
                let codec = LuaProtoCodec;
                match field.runtime_field_type() {
                    RuntimeFieldType::Singular(_) => {
                        match field.get_singular(this.message.as_ref()) {
                            Some(value) => Ok(codec.unbox_value(
                                descriptor.full_name(),
                                field.name(),
                                value,
                                lua,
                                options,
                            )?),
                            None => Ok(Value::Nil),
                        }
                    }
                    RuntimeFieldType::Repeated(_) => {
                        let table = lua.create_table()?;
                        for value in field.get_repeated(this.message.as_ref()) {
                            table.push(codec.unbox_value(
                                descriptor.full_name(),
                                field.name(),
                                value,
                                lua,
                                options,
                            )?)?;
                        }
                        Ok(Value::Table(table))
                    }
                    RuntimeFieldType::Map(_, _) => {
                        let table = lua.create_table()?;
                        let map = field.get_map(this.message.as_ref());
                        for (key, value) in &map {
                            let key = codec.unbox_value(
                                descriptor.full_name(),
                                field.name(),
                                key,
                                lua,
                                options,
                            )?;
                            let value = codec.unbox_value(
                                descriptor.full_name(),
                                field.name(),
                                value,
                                lua,
                                options,
                            )?;
                            table.set(key, value)?;
                        }
                        Ok(Value::Table(table))
                    }
                }
            },
        );

        methods.add_method_mut(
            "set",
            |_, this, (field_name, value, options): (String, Value, Option<Table>)| {
                let mut options = Self::codec_options(options)?;
                options.unknown_fields = UnknownFieldMode::Error;
                let descriptor = this.message.descriptor_dyn();
                let field = descriptor
                    .field_by_name_or_json_name(&field_name)
                    .ok_or(anyhow!(
                        "field {} not found in {}",
                        field_name,
                        descriptor.full_name()
                    ))?;
                let codec = LuaProtoCodec;
                field.clear_field(this.message.as_mut());
                if value.is_nil() {
                    return Ok(());
                }
                match field.runtime_field_type() {
                    RuntimeFieldType::Singular(ty) => {
                        let value = codec.box_value(
                            descriptor.full_name(),
                            field.name(),
                            &ty,
                            value,
                            options,
                        )?;
                        field.set_singular_field(this.message.as_mut(), value);
                    }
                    RuntimeFieldType::Repeated(ty) => {
                        let table = value.as_table().ok_or(anyhow!(
                            "message {} field {} expects a table",
                            descriptor.full_name(),
                            field.name()
                        ))?;
                        let mut repeated = field.mut_repeated(this.message.as_mut());
                        for value in table.sequence_values::<Value>() {
                            let value = codec.box_value(
                                descriptor.full_name(),
                                field.name(),
                                &ty,
                                value?,
                                options,
                            )?;
                            repeated.push(value);
                        }
                    }
                    RuntimeFieldType::Map(key_ty, value_ty) => {
                        let table = value.as_table().ok_or(anyhow!(
                            "message {} field {} expects a table",
                            descriptor.full_name(),
                            field.name()
                        ))?;
                        let mut map = field.mut_map(this.message.as_mut());
                        for pair in table.pairs::<Value, Value>() {
                            let (key, value) = pair?;
                            let key = codec.box_value(
                                descriptor.full_name(),
                                field.name(),
                                &key_ty,
                                key,
                                options,
                            )?;
                            let value = codec.box_value(
                                descriptor.full_name(),
                                field.name(),
                                &value_ty,
                                value,
                                options,
                            )?;
                            map.insert(key, value);
                        }
                    }
                }
                Ok(())
            },
        );

        methods.add_method_mut("clear_oneof", |_, this, oneof_name: String| {
            let descriptor = this.message.descriptor_dyn();
            let oneof = descriptor
                .oneofs()
                .find(|oneof| oneof.name() == oneof_name)
                .ok_or(anyhow!(
                    "oneof {} not found in {}",
                    oneof_name,
                    descriptor.full_name()
                ))?;
            for field in oneof.fields() {
                field.clear_field(this.message.as_mut());
            }
            Ok(())
        });

        methods.add_method_mut(
            "merge",
            |_, this, (lua_message, options): (Table, Option<Table>)| {
                let options = Self::codec_options(options)?;
                let codec = LuaProtoCodec;
                codec
                    .merge_message(&lua_message, this.message.as_mut(), options)
                    .map_err(|e| anyhow!("{e:?}"))?;
                Ok(())
            },
        );

        methods.add_method_mut("clear", |_, this, field_name: Option<String>| {
            let Some(field_name) = field_name else {
                for field in this.message.descriptor_dyn().fields() {
                    field.clear_field(this.message.as_mut());
                }
                return Ok(());
            };
            let descriptor = this.message.descriptor_dyn();
            let field = descriptor
                .field_by_name_or_json_name(&field_name)
                .ok_or(anyhow!(
                    "field {} not found in {}",
                    field_name,
                    descriptor.full_name()
                ))?;
            field.clear_field(this.message.as_mut());
            Ok(())
        });

        methods.add_method("to_table", |lua, this, options: Option<Table>| {
            let options = Self::codec_options(options)?;
            let codec = LuaProtoCodec;
            codec
                .decode_message(lua, this.message.as_ref(), options)
                .map_err(|e| anyhow!("{e:?}").into())
        });

        methods.add_method("validate", |_, this, ()| {
            let codec = LuaProtoCodec;
            match codec.check_required_fields(this.message.as_ref()) {
                Ok(()) => Ok((true, None::<String>)),
                Err(e) => Ok((false, Some(format!("{e:?}")))),
            }
        });

        methods.add_method("encode", |lua, this, ()| {
            let codec = LuaProtoCodec;
            codec
                .check_required_fields(this.message.as_ref())
                .map_err(|e| anyhow!("{e:?}"))?;
            let mut bytes = Vec::with_capacity(this.message.compute_size_dyn() as usize);
            this.message
                .write_to_vec_dyn(&mut bytes)
                .map_err(|e| anyhow!("{e:?}"))?;
            lua.create_string(bytes)
        });
    }
}
