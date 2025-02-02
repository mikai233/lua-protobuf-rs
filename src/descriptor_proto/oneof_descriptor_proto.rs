use crate::descriptor_proto::oneof_options::LuaOneofOptions;
use crate::{add_message_dyn_trait_method, add_message_full_trait_method, add_message_trait_method};
use derive_more::{Deref, DerefMut, From, Into};
use mlua::prelude::LuaUserData;
use mlua::{MetaMethod, UserDataFields, UserDataMethods};
use protobuf::descriptor::OneofDescriptorProto;

#[derive(PartialEq, Clone, Default, Debug, Deref, DerefMut, From, Into)]
pub struct LuaOneofDescriptorProto(pub OneofDescriptorProto);

impl LuaUserData for LuaOneofDescriptorProto {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("name", |_, this| {
            Ok(this.name.clone())
        });

        fields.add_field_method_get("options", |_, this| {
            let options: Option<LuaOneofOptions> = this.options.clone().into_option().map(Into::into);
            Ok(options)
        });
    }

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
            Ok(this.to_string())
        });

        methods.add_method("name", |_, this, ()| {
            Ok(this.name().to_string())
        });

        methods.add_method_mut("clear_name", |_, this, ()| {
            this.clear_name();
            Ok(())
        });

        methods.add_method("has_name", |_, this, ()| {
            Ok(this.has_name())
        });

        methods.add_method_mut("set_name", |_, this, v: String| {
            this.set_name(v);
            Ok(())
        });

        methods.add_method_mut("mut_name", |_, this, ()| {
            Ok(this.mut_name().clone())
        });

        methods.add_method_mut("take_name", |_, this, ()| {
            Ok(this.take_name())
        });

        add_message_trait_method!(methods, OneofDescriptorProto, LuaOneofDescriptorProto);

        add_message_dyn_trait_method!(methods, OneofDescriptorProto, LuaOneofDescriptorProto);

        add_message_full_trait_method!(methods, OneofDescriptorProto, LuaOneofDescriptorProto);
    }
}