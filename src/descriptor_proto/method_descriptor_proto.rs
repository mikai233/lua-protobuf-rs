use crate::descriptor_proto::method_options::LuaMethodOptions;
use crate::{
    add_message_dyn_trait_method, add_message_full_trait_method, add_message_trait_method,
};
use derive_more::{Deref, DerefMut, From, Into};
use mlua::prelude::LuaUserData;
use mlua::{MetaMethod, UserDataFields, UserDataMethods};
use protobuf::descriptor::MethodDescriptorProto;

#[derive(PartialEq, Clone, Default, Debug, Deref, DerefMut, From, Into)]
pub struct LuaMethodDescriptorProto(pub MethodDescriptorProto);

impl LuaUserData for LuaMethodDescriptorProto {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("name", |_, this| Ok(this.name.clone()));

        fields.add_field_method_get("input_type", |_, this| Ok(this.input_type.clone()));

        fields.add_field_method_get("output_type", |_, this| Ok(this.output_type.clone()));

        fields.add_field_method_get("options", |_, this| {
            let options: Option<LuaMethodOptions> =
                this.options.clone().into_option().map(Into::into);
            Ok(options)
        });

        fields.add_field_method_get("client_streaming", |_, this| {
            Ok(this.client_streaming.clone())
        });

        fields.add_field_method_get("server_streaming", |_, this| {
            Ok(this.server_streaming.clone())
        });
    }

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| Ok(this.to_string()));

        methods.add_method("name", |_, this, ()| Ok(this.name().to_string()));

        methods.add_method_mut("clear_name", |_, this, ()| {
            this.clear_name();
            Ok(())
        });

        methods.add_method("has_name", |_, this, ()| Ok(this.has_name()));

        methods.add_method_mut("set_name", |_, this, value: String| {
            this.set_name(value);
            Ok(())
        });

        methods.add_method_mut("mut_name", |_, this, ()| Ok(this.mut_name().clone()));

        methods.add_method_mut("take_name", |_, this, ()| Ok(this.take_name()));

        methods.add_method("input_type", |_, this, ()| {
            Ok(this.input_type().to_string())
        });

        methods.add_method_mut("clear_input_type", |_, this, ()| {
            this.clear_input_type();
            Ok(())
        });

        methods.add_method("has_input_type", |_, this, ()| Ok(this.has_input_type()));

        methods.add_method_mut("set_input_type", |_, this, value: String| {
            this.set_input_type(value);
            Ok(())
        });

        methods.add_method_mut("mut_input_type", |_, this, ()| {
            Ok(this.mut_input_type().clone())
        });

        methods.add_method_mut("take_input_type", |_, this, ()| Ok(this.take_input_type()));

        methods.add_method("output_type", |_, this, ()| {
            Ok(this.output_type().to_string())
        });

        methods.add_method_mut("clear_output_type", |_, this, ()| {
            this.clear_output_type();
            Ok(())
        });

        methods.add_method("has_output_type", |_, this, ()| Ok(this.has_output_type()));

        methods.add_method_mut("set_output_type", |_, this, value: String| {
            this.set_output_type(value);
            Ok(())
        });

        methods.add_method_mut("mut_output_type", |_, this, ()| {
            Ok(this.mut_output_type().clone())
        });

        methods.add_method_mut(
            "take_output_type",
            |_, this, ()| Ok(this.take_output_type()),
        );

        methods.add_method(
            "client_streaming",
            |_, this, ()| Ok(this.client_streaming()),
        );

        methods.add_method_mut("clear_client_streaming", |_, this, ()| {
            this.clear_client_streaming();
            Ok(())
        });

        methods.add_method("has_client_streaming", |_, this, ()| {
            Ok(this.has_client_streaming())
        });

        methods.add_method_mut("set_client_streaming", |_, this, value: bool| {
            this.set_client_streaming(value);
            Ok(())
        });

        methods.add_method(
            "server_streaming",
            |_, this, ()| Ok(this.server_streaming()),
        );

        methods.add_method_mut("clear_server_streaming", |_, this, ()| {
            this.clear_server_streaming();
            Ok(())
        });

        methods.add_method("has_server_streaming", |_, this, ()| {
            Ok(this.has_server_streaming())
        });

        methods.add_method_mut("set_server_streaming", |_, this, value: bool| {
            this.set_server_streaming(value);
            Ok(())
        });

        add_message_trait_method!(methods, MethodDescriptorProto, LuaMethodDescriptorProto);

        add_message_dyn_trait_method!(methods, MethodDescriptorProto, LuaMethodDescriptorProto);

        add_message_full_trait_method!(methods, MethodDescriptorProto, LuaMethodDescriptorProto);
    }
}
