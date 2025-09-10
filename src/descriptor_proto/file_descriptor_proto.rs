use crate::descriptor_proto::descriptor_proto::LuaDescriptorProto;
use crate::descriptor_proto::enum_descriptor_proto::LuaEnumDescriptorProto;
use crate::descriptor_proto::field_descriptor_proto::LuaFieldDescriptorProto;
use crate::descriptor_proto::file_options::LuaFileOptions;
use crate::descriptor_proto::service_descriptor_proto::LuaServiceDescriptorProto;
use crate::descriptor_proto::source_code_info::LuaSourceCodeInfo;
use crate::{
    add_message_dyn_trait_method, add_message_full_trait_method, add_message_trait_method,
};
use derive_more::{Deref, DerefMut, From, Into};
use mlua::prelude::LuaUserData;
use mlua::{MetaMethod, UserDataFields, UserDataMethods};
use protobuf::descriptor::FileDescriptorProto;

#[derive(PartialEq, Clone, Default, Debug, Deref, DerefMut, From, Into, mlua::FromLua)]
pub struct LuaFileDescriptorProto(FileDescriptorProto);

impl LuaUserData for LuaFileDescriptorProto {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("name", |_, this| Ok(this.name.clone()));

        fields.add_field_method_get("package", |_, this| Ok(this.package.clone()));

        fields.add_field_method_get("dependency", |_, this| Ok(this.dependency.clone()));

        fields.add_field_method_get("public_dependency", |_, this| {
            Ok(this.public_dependency.clone())
        });

        fields.add_field_method_get("weak_dependency", |_, this| {
            Ok(this.weak_dependency.clone())
        });

        fields.add_field_method_get("message_type", |_, this| {
            let message_type = this
                .message_type
                .iter()
                .map(Clone::clone)
                .map(Into::into)
                .collect::<Vec<LuaDescriptorProto>>();
            Ok(message_type)
        });

        fields.add_field_method_get("enum_type", |_, this| {
            let enum_type = this
                .enum_type
                .iter()
                .map(Clone::clone)
                .map(Into::into)
                .collect::<Vec<LuaEnumDescriptorProto>>();
            Ok(enum_type)
        });

        fields.add_field_method_get("service", |_, this| {
            let service = this
                .service
                .iter()
                .map(Clone::clone)
                .map(Into::into)
                .collect::<Vec<LuaServiceDescriptorProto>>();
            Ok(service)
        });

        fields.add_field_method_get("extension", |_, this| {
            let extension = this
                .extension
                .iter()
                .map(Clone::clone)
                .map(Into::into)
                .collect::<Vec<LuaFieldDescriptorProto>>();
            Ok(extension)
        });

        fields.add_field_method_get("options", |_, this| {
            let options: Option<LuaFileOptions> =
                this.options.clone().into_option().map(Into::into);
            Ok(options)
        });

        fields.add_field_method_get("source_code_info", |_, this| {
            let source_code_info: Option<LuaSourceCodeInfo> =
                this.source_code_info.clone().into_option().map(Into::into);
            Ok(source_code_info)
        });

        fields.add_field_method_get("syntax", |_, this| Ok(this.syntax.clone()));
    }

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| Ok(this.to_string()));

        methods.add_method("name", |_, this, ()| Ok(this.name().to_string()));

        methods.add_method("has_name", |_, this, ()| Ok(this.has_name()));

        methods.add_method_mut("clear_name", |_, this, ()| {
            this.clear_name();
            Ok(())
        });

        methods.add_method_mut("set_name", |_, this, value: String| {
            this.set_name(value);
            Ok(())
        });

        methods.add_method_mut("mut_name", |_, this, ()| Ok(this.mut_name().clone()));

        methods.add_method_mut("take_name", |_, this, ()| Ok(this.take_name()));

        methods.add_method("package", |_, this, ()| Ok(this.package().to_string()));

        methods.add_method("has_package", |_, this, ()| Ok(this.has_package()));

        methods.add_method_mut("clear_package", |_, this, ()| {
            this.clear_package();
            Ok(())
        });

        methods.add_method_mut("set_package", |_, this, value: String| {
            this.set_package(value);
            Ok(())
        });

        methods.add_method_mut("mut_package", |_, this, ()| Ok(this.mut_package().clone()));

        methods.add_method_mut("take_package", |_, this, ()| Ok(this.take_package()));

        methods.add_method("syntax", |_, this, ()| Ok(this.syntax().to_string()));

        methods.add_method("has_syntax", |_, this, ()| Ok(this.has_syntax()));

        methods.add_method_mut("clear_syntax", |_, this, ()| {
            this.clear_syntax();
            Ok(())
        });

        methods.add_method_mut("set_syntax", |_, this, value: String| {
            this.set_syntax(value);
            Ok(())
        });

        methods.add_method_mut("mut_syntax", |_, this, ()| Ok(this.mut_syntax().clone()));

        methods.add_method_mut("take_syntax", |_, this, ()| Ok(this.take_syntax()));

        add_message_trait_method!(methods, FileDescriptorProto, LuaFileDescriptorProto);

        add_message_dyn_trait_method!(methods, FileDescriptorProto, LuaFileDescriptorProto);

        add_message_full_trait_method!(methods, FileDescriptorProto, LuaFileDescriptorProto);
    }
}
