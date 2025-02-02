use crate::{add_message_dyn_trait_method, add_message_full_trait_method, add_message_trait_method};
use derive_more::{Deref, DerefMut, From, Into};
use mlua::prelude::LuaUserData;
use mlua::{MetaMethod, UserDataMethods};
use protobuf::descriptor::ServiceDescriptorProto;

#[derive(PartialEq, Clone, Default, Debug, Deref, DerefMut, From, Into)]
pub struct LuaServiceDescriptorProto(pub ServiceDescriptorProto);

impl LuaUserData for LuaServiceDescriptorProto {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
            Ok(this.to_string())
        });

        methods.add_method("name", |_, this, ()| {
            Ok(this.name().to_string())
        });

        methods.add_method("has_name", |_, this, ()| {
            Ok(this.has_name())
        });

        methods.add_method_mut("clear_name", |_, this, ()| {
            this.clear_name();
            Ok(())
        });

        methods.add_method_mut("set_name", |_, this, value: String| {
            this.set_name(value);
            Ok(())
        });

        methods.add_method_mut("mut_name", |_, this, ()| {
            Ok(this.mut_name().clone())
        });

        methods.add_method_mut("take_name", |_, this, ()| {
            Ok(this.take_name())
        });

        add_message_trait_method!(methods, ServiceDescriptorProto, LuaServiceDescriptorProto);

        add_message_dyn_trait_method!(methods, ServiceDescriptorProto, LuaServiceDescriptorProto);

        add_message_full_trait_method!(methods, ServiceDescriptorProto, LuaServiceDescriptorProto);
    }
}