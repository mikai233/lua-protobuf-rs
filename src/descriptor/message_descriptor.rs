use std::ops::{Deref, DerefMut};
use mlua::prelude::LuaUserData;
use mlua::UserDataMethods;
use protobuf::reflect::MessageDescriptor;
use crate::descriptor::enum_descriptor::LuaEnumDescriptor;
use crate::descriptor::field_descriptor::LuaFieldDescriptor;
use crate::descriptor::file_descriptor::LuaFileDescriptor;
use crate::descriptor::oneof_descriptor::LuaOneofDescriptor;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct LuaMessageDescriptor(MessageDescriptor);

impl Deref for LuaMessageDescriptor {
    type Target = MessageDescriptor;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaMessageDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<MessageDescriptor> for LuaMessageDescriptor {
    fn from(value: MessageDescriptor) -> Self {
        LuaMessageDescriptor(value)
    }
}

impl LuaUserData for LuaMessageDescriptor {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("name", |_, this, ()| {
            Ok(this.name().to_string())
        });
        methods.add_method("nested_messages", |_, this, ()| {
            let nested_messages: Vec<LuaMessageDescriptor> = this.nested_messages().map(From::from).collect();
            Ok(nested_messages)
        });
        methods.add_method("nested_enums", |_, this, ()| {
            let nested_enums: Vec<LuaEnumDescriptor> = this.nested_enums().map(From::from).collect();
            Ok(nested_enums)
        });
        methods.add_method("enclosing_message", |_, this, ()| {
            let enclosing_message: Option<LuaMessageDescriptor> = this.enclosing_message().map(From::from);
            Ok(enclosing_message)
        });
        methods.add_method("file_descriptor", |_, this, ()| {
            let file_descriptor: LuaFileDescriptor = From::from(this.file_descriptor().clone());
            Ok(file_descriptor)
        });
        methods.add_method("is_map_entry", |_, this, ()| {
            Ok(this.is_map_entry())
        });
        methods.add_method("full_name", |_, this, ()| {
            Ok(this.full_name().to_string())
        });
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
            let descriptor: Option<LuaOneofDescriptor> = this.oneof_by_name(name.as_str()).map(From::from);
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
            let field_descriptor: Option<LuaFieldDescriptor> = this.field_by_name(name.as_str()).map(From::from);
            Ok(field_descriptor)
        });
        methods.add_method("field_by_number", |_, this, number: u32| {
            let field_descriptor: Option<LuaFieldDescriptor> = this.field_by_number(number).map(From::from);
            Ok(field_descriptor)
        });
    }
}