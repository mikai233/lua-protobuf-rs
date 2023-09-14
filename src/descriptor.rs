use std::ops::{Deref, DerefMut};
use mlua::prelude::LuaUserData;
use mlua::UserDataMethods;
use protobuf::MessageDyn;
use protobuf::reflect::{EnumDescriptor, EnumValueDescriptor, FieldDescriptor, FileDescriptor, MessageDescriptor, OneofDescriptor, RuntimeFieldType, RuntimeType, Syntax};
use crate::field_descriptor_proto::LuaFieldDescriptorProto;
use crate::runtime_field_type::LuaRuntimeFieldType;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
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
        methods.add_method("deps", |_, this, name: String| {
            let descriptors: Vec<LuaFileDescriptor> = this.deps().iter().map(Clone::clone).map(From::from).collect();
            Ok(descriptors)
        });
        methods.add_method("public_deps", |_, this, name: String| {
            let descriptors: Vec<LuaFileDescriptor> = this.public_deps().map(From::from).collect();
            Ok(descriptors)
        });
    }
}

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

#[derive(Clone, Eq, PartialEq)]
pub struct LuaFieldDescriptor(FieldDescriptor);

impl Deref for LuaFieldDescriptor {
    type Target = FieldDescriptor;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaFieldDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<FieldDescriptor> for LuaFieldDescriptor {
    fn from(value: FieldDescriptor) -> Self {
        LuaFieldDescriptor(value)
    }
}

impl LuaUserData for LuaFieldDescriptor {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("proto", |_, this, ()| {
            let proto: LuaFieldDescriptorProto = From::from(this.proto().clone());
            Ok(proto)
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
            let descriptor: Option<LuaOneofDescriptor> = this.containing_oneof_including_synthetic().map(From::from);
            Ok(descriptor)
        });
        methods.add_method("containing_oneof", |_, this, ()| {
            let descriptor: Option<LuaOneofDescriptor> = this.containing_oneof().map(From::from);
            Ok(descriptor)
        });
        methods.add_method("containing_message", |_, this, ()| {
            let descriptor: LuaMessageDescriptor = From::from(this.containing_message());
            Ok(descriptor)
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
        methods.add_method("runtime_field_type", |_, this, ()| {
            let ty: LuaRuntimeFieldType = From::from(this.runtime_field_type());
            Ok(ty)
        });
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct LuaEnumDescriptor(EnumDescriptor);

impl Deref for LuaEnumDescriptor {
    type Target = EnumDescriptor;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaEnumDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<EnumDescriptor> for LuaEnumDescriptor {
    fn from(value: EnumDescriptor) -> Self {
        LuaEnumDescriptor(value)
    }
}

impl LuaUserData for LuaEnumDescriptor {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("name", |_, this, ()| {
            Ok(this.name().to_string())
        });
        methods.add_method("full_name", |_, this, ()| {
            Ok(this.full_name().to_string())
        });
        methods.add_method("name_to_package", |_, this, ()| {
            Ok(this.name_to_package().to_string())
        });
        methods.add_method("enclosing_message", |_, this, ()| {
            let enclosing_message: Option<LuaMessageDescriptor> = this.enclosing_message().map(From::from);
            Ok(enclosing_message)
        });
        methods.add_method("values", |_, this, ()| {
            let values: Vec<LuaEnumValueDescriptor> = this.values().map(From::from).collect();
            Ok(values)
        });
        methods.add_method("value_by_name", |_, this, name: String| {
            let descriptor: Option<LuaEnumValueDescriptor> = this.value_by_name(name.as_str()).map(From::from);
            Ok(descriptor)
        });
        methods.add_method("value_by_number", |_, this, number: i32| {
            let descriptor: Option<LuaEnumValueDescriptor> = this.value_by_number(number).map(From::from);
            Ok(descriptor)
        });
        methods.add_method("value_by_number", |_, this, index: usize| {
            let descriptor: LuaEnumValueDescriptor = From::from(this.value_by_index(index));
            Ok(descriptor)
        });
        methods.add_method("default_value", |_, this, ()| {
            let descriptor: LuaEnumValueDescriptor = From::from(this.default_value());
            Ok(descriptor)
        });
        methods.add_method("value_by_number_or_default", |_, this, number: i32| {
            let descriptor: LuaEnumValueDescriptor = From::from(this.value_by_number_or_default(number));
            Ok(descriptor)
        });
    }
}

pub struct LuaOneofDescriptor(OneofDescriptor);

impl Deref for LuaOneofDescriptor {
    type Target = OneofDescriptor;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaOneofDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<OneofDescriptor> for LuaOneofDescriptor {
    fn from(value: OneofDescriptor) -> Self {
        LuaOneofDescriptor(value)
    }
}

impl LuaUserData for LuaOneofDescriptor {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("name", |_, this, ()| {
            Ok(this.name().to_string())
        });
        methods.add_method("containing_message", |_, this, ()| {
            let descriptor: LuaMessageDescriptor = From::from(this.containing_message());
            Ok(descriptor)
        });
        methods.add_method("is_synthetic", |_, this, ()| {
            Ok(this.is_synthetic())
        });
        methods.add_method("full_name", |_, this, ()| {
            Ok(this.full_name())
        });
        methods.add_method("fields", |_, this, ()| {
            let descriptors: Vec<LuaFieldDescriptor> = this.fields().map(From::from).collect();
            Ok(descriptors)
        });
    }
}

pub struct LuaEnumValueDescriptor(EnumValueDescriptor);

impl Deref for LuaEnumValueDescriptor {
    type Target = EnumValueDescriptor;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaEnumValueDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<EnumValueDescriptor> for LuaEnumValueDescriptor {
    fn from(value: EnumValueDescriptor) -> Self {
        LuaEnumValueDescriptor(value)
    }
}

impl LuaUserData for LuaEnumValueDescriptor {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("name", |_, this, ()| {
            Ok(this.name().to_string())
        });
        methods.add_method("full_name", |_, this, ()| {
            Ok(this.full_name().to_string())
        });
        methods.add_method("value", |_, this, ()| {
            Ok(this.value())
        });
        methods.add_method("enum_descriptor", |_, this, ()| {
            let descriptor: LuaEnumDescriptor = From::from(this.enum_descriptor().clone());
            Ok(descriptor)
        });
    }
}