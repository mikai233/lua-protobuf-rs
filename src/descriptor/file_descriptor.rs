use std::ops::{Deref, DerefMut};

use mlua::{ExternalError, UserDataMethods};
use mlua::prelude::LuaUserData;
use protobuf::reflect::{FileDescriptor, Syntax};

use crate::descriptor::enum_descriptor::LuaEnumDescriptor;
use crate::descriptor::field_descriptor::LuaFieldDescriptor;
use crate::descriptor::message_descriptor::LuaMessageDescriptor;
use crate::descriptor::service_descriptor::LuaServiceDescriptor;
use crate::descriptor_proto::file_descriptor_proto::LuaFileDescriptorProto;

#[derive(Clone, PartialEq, Eq, Hash, Debug, mlua::FromLua)]
pub struct LuaFileDescriptor(FileDescriptor);

impl Deref for LuaFileDescriptor {
    type Target = FileDescriptor;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaFileDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<FileDescriptor> for LuaFileDescriptor {
    fn from(value: FileDescriptor) -> Self {
        LuaFileDescriptor(value)
    }
}

impl Into<FileDescriptor> for LuaFileDescriptor {
    fn into(self) -> FileDescriptor {
        self.0
    }
}

impl LuaUserData for LuaFileDescriptor {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("name", |_, this, ()| {
            Ok(this.name().to_string())
        });
        methods.add_method("package", |_, this, ()| {
            Ok(this.package().to_string())
        });
        methods.add_method("syntax", |_, this, ()| {
            let syntax = match this.syntax() {
                Syntax::Proto2 => { "proto2" }
                Syntax::Proto3 => { "proto3" }
            };
            Ok(syntax.to_string())
        });
        methods.add_method("messages", |_, this, ()| {
            let messages: Vec<LuaMessageDescriptor> = this.messages().map(From::from).collect();
            Ok(messages)
        });
        methods.add_method("enums", |_, this, ()| {
            let enums: Vec<LuaEnumDescriptor> = this.enums().map(From::from).collect();
            Ok(enums)
        });
        methods.add_method("services", |_, this, ()| {
            let services: Vec<LuaServiceDescriptor> = this.services().map(From::from).collect();
            Ok(services)
        });
        methods.add_method("extensions", |_, this, ()| {
            let descriptors: Vec<LuaFieldDescriptor> = this.extensions().map(From::from).collect();
            Ok(descriptors)
        });
        methods.add_method("message_by_package_relative_name", |_, this, name: String| {
            let descriptor: Option<LuaMessageDescriptor> = this.message_by_package_relative_name(name.as_str()).map(From::from);
            Ok(descriptor)
        });
        methods.add_method("enum_by_package_relative_name", |_, this, name: String| {
            let descriptor: Option<LuaEnumDescriptor> = this.enum_by_package_relative_name(name.as_str()).map(From::from);
            Ok(descriptor)
        });
        methods.add_method("message_by_full_name", |_, this, name: String| {
            let descriptor: Option<LuaMessageDescriptor> = this.message_by_full_name(name.as_str()).map(From::from);
            Ok(descriptor)
        });
        methods.add_method("enum_by_full_name", |_, this, name: String| {
            let descriptor: Option<LuaEnumDescriptor> = this.enum_by_full_name(name.as_str()).map(From::from);
            Ok(descriptor)
        });
        methods.add_function("new_dynamic", |_, (proto, dependencies): (LuaFileDescriptorProto, Vec<LuaFileDescriptor>)| {
            let descriptor: LuaFileDescriptor = FileDescriptor::new_dynamic(
                proto.into(),
                dependencies.into_iter().map(Into::into).collect::<Vec<FileDescriptor>>().as_slice())
                .map_err(|e| e.into_lua_err())?.into();
            Ok(descriptor)
        });
        methods.add_function("new_dynamic_fds", |_, (protos, dependencies): (Vec<LuaFileDescriptorProto>, Vec<LuaFileDescriptor>)| {
            let descriptors: Vec<LuaFileDescriptor> = FileDescriptor::new_dynamic_fds(
                protos.into_iter().map(Into::into).collect(),
                dependencies.into_iter().map(Into::into).collect::<Vec<FileDescriptor>>().as_slice())
                .map_err(|e| e.into_lua_err())?.into_iter().map(Into::into).collect();
            Ok(descriptors)
        });
        methods.add_method("proto", |_, this, ()| {
            let proto: LuaFileDescriptorProto = this.proto().clone().into();
            Ok(proto)
        });
        methods.add_method("deps", |_, this, ()| {
            let descriptors: Vec<LuaFileDescriptor> = this.deps().iter().map(Clone::clone).map(From::from).collect();
            Ok(descriptors)
        });
        methods.add_method("public_deps", |_, this, ()| {
            let descriptors: Vec<LuaFileDescriptor> = this.public_deps().map(From::from).collect();
            Ok(descriptors)
        });
    }
}