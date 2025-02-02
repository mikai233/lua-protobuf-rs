use anyhow::anyhow;
use derive_more::{Deref, From, Into};
use mlua::prelude::LuaUserData;
use mlua::UserDataMethods;
use protobuf::reflect::FileDescriptor;

use crate::descriptor::enum_descriptor::LuaEnumDescriptor;
use crate::descriptor::field_descriptor::LuaFieldDescriptor;
use crate::descriptor::message_descriptor::LuaMessageDescriptor;
use crate::descriptor::service_descriptor::LuaServiceDescriptor;
use crate::descriptor_proto::file_descriptor_proto::LuaFileDescriptorProto;
use crate::syntax::LuaSyntax;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Deref, From, Into, mlua::FromLua)]
pub struct LuaFileDescriptor(pub FileDescriptor);

impl LuaUserData for LuaFileDescriptor {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("name", |_, this, ()| {
            Ok(this.name().to_string())
        });

        methods.add_method("package", |_, this, ()| {
            Ok(this.package().to_string())
        });
        
        methods.add_method("syntax", |_, this, ()| {
            Ok(LuaSyntax(this.syntax()))
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
                .map_err(|e| anyhow!(e))?.into();
            Ok(descriptor)
        });

        methods.add_function("new_dynamic_fds", |_, (protos, dependencies): (Vec<LuaFileDescriptorProto>, Vec<LuaFileDescriptor>)| {
            let descriptors: Vec<LuaFileDescriptor> = FileDescriptor::new_dynamic_fds(
                protos.into_iter().map(Into::into).collect(),
                dependencies.into_iter().map(Into::into).collect::<Vec<FileDescriptor>>().as_slice())
                .map_err(|e| anyhow!(e))?.into_iter().map(Into::into).collect();
            Ok(descriptors)
        });
        
        methods.add_method("proto", |_, this, ()| {
            Ok::<LuaFileDescriptorProto, _>(this.proto().clone().into())
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