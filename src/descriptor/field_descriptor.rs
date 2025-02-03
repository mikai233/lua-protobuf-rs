use anyhow::anyhow;
use derive_more::{Deref, From, Into};
use mlua::prelude::LuaUserData;
use mlua::{AnyUserData, MetaMethod, UserDataMethods};
use protobuf::reflect::FieldDescriptor;

use crate::descriptor::message_descriptor::LuaMessageDescriptor;
use crate::descriptor::oneof_descriptor::LuaOneofDescriptor;
use crate::descriptor_proto::field_descriptor_proto::LuaFieldDescriptorProto;
use crate::message_dyn::LuaMessageDyn;
use crate::reflect_value_box::LuaReflectedValueBox;
use crate::runtime_field_type::LuaRuntimeFieldType;
use crate::runtime_type::LuaRuntimeType;

#[derive(Clone, Eq, PartialEq, Deref, From, Into)]
pub struct LuaFieldDescriptor(pub FieldDescriptor);

impl LuaUserData for LuaFieldDescriptor {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
            Ok(this.to_string())
        });

        methods.add_method("proto", |_, this, ()| {
            Ok(LuaFieldDescriptorProto(this.proto().clone()))
        });

        methods.add_method("name", |_, this, ()| {
            Ok(this.name().to_string())
        });

        methods.add_method("number", |_, this, ()| {
            Ok(this.number())
        });

        methods.add_method("full_name", |_, this, ()| {
            Ok(this.full_name())
        });
        
        methods.add_method("containing_oneof_including_synthetic", |_, this, ()| {
            let descriptor: Option<LuaOneofDescriptor> = this.containing_oneof_including_synthetic().map(Into::into);
            Ok(descriptor)
        });

        methods.add_method("containing_oneof", |_, this, ()| {
            let descriptor: Option<LuaOneofDescriptor> = this.containing_oneof().map(Into::into);
            Ok(descriptor)
        });

        methods.add_method("containing_message", |_, this, ()| {
            let descriptor: LuaMessageDescriptor = this.containing_message().into();
            Ok(descriptor)
        });

        methods.add_method("json_name", |_, this, ()| {
            Ok(this.json_name().to_string())
        });

        methods.add_method("is_singular", |_, this, ()| {
            Ok(this.is_singular())
        });

        methods.add_method("is_required", |_, this, ()| {
            Ok(this.is_required())
        });
        
        methods.add_method("is_repeated_or_map", |_, this, ()| {
            Ok(this.is_repeated_or_map())
        });

        methods.add_method("is_repeated", |_, this, ()| {
            Ok(this.is_repeated())
        });

        methods.add_method("is_map", |_, this, ()| {
            Ok(this.is_repeated())
        });

        methods.add_method("has_field", |_, this, m: AnyUserData| {
            let m = m.borrow::<LuaMessageDyn>()?;
            Ok(this.has_field(&***m))
        });

        methods.add_method("get_message", |_, this, m: AnyUserData| {
            let m = m.borrow::<LuaMessageDyn>()?;
            let m = this.get_message(&***m);
            let m: LuaMessageDyn = m.clone_box().into();
            Ok(m)
        });

        // mut_message

        methods.add_method("singular_default_value", |_, this, ()| {
            Ok::<LuaReflectedValueBox, _>(this.singular_default_value().to_box().into())
        });
        
        
        methods.add_method("singular_runtime_type", |_, this, ()| {
            if this.is_singular() {
                let ty: LuaRuntimeType = this.singular_runtime_type().into();
                Ok(ty)
            } else {
                Err(anyhow!("{} is not singular",this.full_name()).into())
            }
        });
        methods.add_method("runtime_field_type", |_, this, ()| {
            let ty: LuaRuntimeFieldType = this.runtime_field_type().into();
            Ok(ty)
        });
    }
}