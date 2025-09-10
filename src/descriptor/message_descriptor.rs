use anyhow::anyhow;
use derive_more::{Deref, From, Into};
use mlua::prelude::LuaUserData;
use mlua::{AnyUserData, ErrorContext, MetaMethod, Table, UserDataMethods};
use protobuf::reflect::MessageDescriptor;

use crate::codec::LuaProtoCodec;
use crate::descriptor::enum_descriptor::LuaEnumDescriptor;
use crate::descriptor::field_descriptor::LuaFieldDescriptor;
use crate::descriptor::file_descriptor::LuaFileDescriptor;
use crate::descriptor::oneof_descriptor::LuaOneofDescriptor;
use crate::descriptor_proto::descriptor_proto::LuaDescriptorProto;
use crate::descriptor_proto::file_descriptor_proto::LuaFileDescriptorProto;
use crate::message_dyn::LuaMessageDyn;

#[derive(Clone, Eq, PartialEq, Hash, Deref, From, Into)]
pub struct LuaMessageDescriptor(pub MessageDescriptor);

impl LuaUserData for LuaMessageDescriptor {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method(MetaMethod::ToString, |_, this, ()| Ok(this.to_string()));

        methods.add_meta_method(MetaMethod::Eq, |_, this, other: AnyUserData| {
            let other = other.borrow::<LuaMessageDescriptor>()?;
            Ok(*this == *other)
        });

        methods.add_method("proto", |_, this, ()| {
            let proto: LuaDescriptorProto = this.proto().clone().into();
            Ok(proto)
        });

        methods.add_method("name", |_, this, ()| Ok(this.name().to_string()));

        methods.add_method("nested_messages", |_, this, ()| {
            let nested_messages: Vec<LuaMessageDescriptor> =
                this.nested_messages().map(From::from).collect();
            Ok(nested_messages)
        });

        methods.add_method("nested_enums", |_, this, ()| {
            let nested_enums: Vec<LuaEnumDescriptor> =
                this.nested_enums().map(From::from).collect();
            Ok(nested_enums)
        });

        methods.add_method("enclosing_message", |_, this, ()| {
            let enclosing_message: Option<LuaMessageDescriptor> =
                this.enclosing_message().map(From::from);
            Ok(enclosing_message)
        });

        methods.add_method("file_descriptor", |_, this, ()| {
            let file_descriptor: LuaFileDescriptor = From::from(this.file_descriptor().clone());
            Ok(file_descriptor)
        });

        methods.add_method("file_descriptor_proto", |_, this, ()| {
            let proto: LuaFileDescriptorProto = this.file_descriptor_proto().clone().into();
            Ok(proto)
        });

        methods.add_method("is_map_entry", |_, this, ()| Ok(this.is_map_entry()));

        methods.add_method("new_instance", |_, this, ()| {
            Ok::<LuaMessageDyn, _>(this.new_instance().into())
        });

        methods.add_method("default_instance", |_, this, ()| {
            match this.default_instance() {
                None => Ok(None),
                Some(instance) => Ok::<Option<LuaMessageDyn>, _>(Some(instance.clone_box().into())),
            }
        });

        methods.add_method("clone_message", |_, this, ()| {
            match this.enclosing_message() {
                None => Ok(None),
                Some(message) => {
                    Ok::<Option<LuaMessageDescriptor>, _>(Some(message.clone().into()))
                }
            }
        });

        methods.add_method("eq", |_, this, (a, b): (AnyUserData, AnyUserData)| {
            let a = a.borrow::<LuaMessageDyn>()?;
            let b = b.borrow::<LuaMessageDyn>()?;
            Ok(MessageDescriptor::eq(this, &***a, &***b))
        });

        methods.add_method("full_name", |_, this, ()| Ok(this.full_name().to_string()));

        methods.add_method("name_to_package", |_, this, ()| {
            Ok(this.name_to_package().to_string())
        });

        methods.add_method("all_oneofs", |_, this, ()| {
            let all_oneofs: Vec<LuaOneofDescriptor> = this.all_oneofs().map(From::from).collect();
            Ok(all_oneofs)
        });

        methods.add_method("oneofs", |_, this, ()| {
            let oneofs: Vec<LuaOneofDescriptor> = this.oneofs().map(From::from).collect();
            Ok(oneofs)
        });

        methods.add_method("oneof_by_name", |_, this, name: String| {
            let descriptor: Option<LuaOneofDescriptor> =
                this.oneof_by_name(name.as_str()).map(From::from);
            Ok(descriptor)
        });

        methods.add_method("fields", |_, this, ()| {
            let fields: Vec<LuaFieldDescriptor> = this.fields().map(From::from).collect();
            Ok(fields)
        });

        methods.add_method("extensions", |_, this, ()| {
            let extensions: Vec<LuaFieldDescriptor> = this.extensions().map(From::from).collect();
            Ok(extensions)
        });

        methods.add_method("field_by_name", |_, this, name: String| {
            let field_descriptor: Option<LuaFieldDescriptor> =
                this.field_by_name(name.as_str()).map(From::from);
            Ok(field_descriptor)
        });

        methods.add_method("field_by_name_or_json_name", |_, this, name: String| {
            let field_descriptor: Option<LuaFieldDescriptor> = this
                .field_by_name_or_json_name(name.as_str())
                .map(From::from);
            Ok(field_descriptor)
        });

        methods.add_method("field_by_number", |_, this, number: u32| {
            let field_descriptor: Option<LuaFieldDescriptor> =
                this.field_by_number(number).map(From::from);
            Ok(field_descriptor)
        });

        methods.add_method("parse_from_bytes", |lua, this, bytes_table: Table| {
            let len = bytes_table.len()? as usize;
            let mut bytes = Vec::with_capacity(len);
            for byte in bytes_table.sequence_values::<u8>() {
                bytes.push(byte.context("expect u8 in table, found other type")?);
            }
            let message = this
                .parse_from_bytes(bytes.as_slice())
                .map_err(|e| anyhow!(e))?;
            let codec = LuaProtoCodec::default();
            let message = codec.decode_message(lua, message.as_ref())?;
            Ok(message)
        });
    }
}
