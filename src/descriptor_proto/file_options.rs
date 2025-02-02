use crate::descriptor_proto::uninterpreted_option::LuaUninterpretedOption;
use crate::{add_message_dyn_trait_method, add_message_full_trait_method, add_message_trait_method};
use anyhow::anyhow;
use derive_more::{Deref, DerefMut, From, Into};
use mlua::prelude::LuaUserData;
use mlua::{MetaMethod, UserDataFields, UserDataMethods};
use protobuf::descriptor::file_options::OptimizeMode;
use protobuf::descriptor::FileOptions;
use protobuf::Enum;

#[derive(PartialEq, Clone, Default, Debug, Deref, DerefMut, From, Into)]
pub struct LuaFileOptions(pub FileOptions);

impl LuaUserData for LuaFileOptions {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("java_package", |_, this| {
            Ok(this.java_package.clone())
        });

        fields.add_field_method_get("java_outer_classname", |_, this| {
            Ok(this.java_outer_classname.clone())
        });

        fields.add_field_method_get("java_multiple_files", |_, this| {
            Ok(this.java_multiple_files)
        });

        fields.add_field_method_get("java_generate_equals_and_hash", |_, this| {
            Ok(this.java_generate_equals_and_hash)
        });

        fields.add_field_method_get("java_string_check_utf8", |_, this| {
            Ok(this.java_string_check_utf8)
        });

        fields.add_field_method_get("optimize_for", |_, this| {
            Ok(this.optimize_for.map(|e| e.value()))
        });

        fields.add_field_method_get("go_package", |_, this| {
            Ok(this.go_package.clone())
        });

        fields.add_field_method_get("cc_generic_services", |_, this| {
            Ok(this.cc_generic_services)
        });

        fields.add_field_method_get("java_generic_services", |_, this| {
            Ok(this.java_generic_services)
        });

        fields.add_field_method_get("py_generic_services", |_, this| {
            Ok(this.py_generic_services)
        });

        fields.add_field_method_get("php_generic_services", |_, this| {
            Ok(this.php_generic_services)
        });

        fields.add_field_method_get("deprecated", |_, this| {
            Ok(this.deprecated)
        });

        fields.add_field_method_get("cc_enable_arenas", |_, this| {
            Ok(this.cc_enable_arenas)
        });

        fields.add_field_method_get("objc_class_prefix", |_, this| {
            Ok(this.objc_class_prefix.clone())
        });

        fields.add_field_method_get("csharp_namespace", |_, this| {
            Ok(this.csharp_namespace.clone())
        });

        fields.add_field_method_get("swift_prefix", |_, this| {
            Ok(this.swift_prefix.clone())
        });

        fields.add_field_method_get("php_class_prefix", |_, this| {
            Ok(this.php_class_prefix.clone())
        });

        fields.add_field_method_get("php_namespace", |_, this| {
            Ok(this.php_namespace.clone())
        });

        fields.add_field_method_get("php_metadata_namespace", |_, this| {
            Ok(this.php_metadata_namespace.clone())
        });

        fields.add_field_method_get("ruby_package", |_, this| {
            Ok(this.ruby_package.clone())
        });

        fields.add_field_method_get("uninterpreted_option", |_, this| {
            let uninterpreted_option = this.uninterpreted_option.iter().map(Clone::clone).map(Into::into).collect::<Vec<LuaUninterpretedOption>>();
            Ok(uninterpreted_option)
        });
    }

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
            Ok(this.to_string())
        });

        methods.add_method("java_package", |_, this, ()| {
            Ok(this.java_package().to_string())
        });

        methods.add_method("has_java_package", |_, this, ()| {
            Ok(this.has_java_package())
        });

        methods.add_method_mut("clear_java_package", |_, this, ()| {
            this.clear_java_package();
            Ok(())
        });

        methods.add_method_mut("set_java_package", |_, this, v: String| {
            this.set_java_package(v);
            Ok(())
        });

        methods.add_method_mut("mut_java_package", |_, this, ()| {
            Ok(this.mut_java_package().clone())
        });

        methods.add_method_mut("take_java_package", |_, this, ()| {
            Ok(this.take_java_package())
        });

        methods.add_method("java_outer_classname", |_, this, ()| {
            Ok(this.java_outer_classname().to_string())
        });

        methods.add_method("has_java_outer_classname", |_, this, ()| {
            Ok(this.has_java_outer_classname())
        });

        methods.add_method_mut("clear_java_outer_classname", |_, this, ()| {
            this.clear_java_outer_classname();
            Ok(())
        });

        methods.add_method_mut("set_java_outer_classname", |_, this, v: String| {
            this.set_java_outer_classname(v);
            Ok(())
        });

        methods.add_method_mut("mut_java_outer_classname", |_, this, ()| {
            Ok(this.mut_java_outer_classname().clone())
        });

        methods.add_method_mut("take_java_outer_classname", |_, this, ()| {
            Ok(this.take_java_outer_classname())
        });

        methods.add_method("java_multiple_files", |_, this, ()| {
            Ok(this.java_multiple_files())
        });

        methods.add_method("has_java_multiple_files", |_, this, ()| {
            Ok(this.has_java_multiple_files())
        });

        methods.add_method_mut("clear_java_multiple_files", |_, this, ()| {
            this.clear_java_multiple_files();
            Ok(())
        });

        methods.add_method_mut("set_java_multiple_files", |_, this, v: bool| {
            this.set_java_multiple_files(v);
            Ok(())
        });

        methods.add_method("java_generate_equals_and_hash", |_, this, ()| {
            Ok(this.java_generate_equals_and_hash())
        });

        methods.add_method("has_java_generate_equals_and_hash", |_, this, ()| {
            Ok(this.has_java_generate_equals_and_hash())
        });

        methods.add_method_mut("clear_java_generate_equals_and_hash", |_, this, ()| {
            this.clear_java_generate_equals_and_hash();
            Ok(())
        });

        methods.add_method_mut("set_java_generate_equals_and_hash", |_, this, v: bool| {
            this.set_java_generate_equals_and_hash(v);
            Ok(())
        });

        methods.add_method("java_string_check_utf8", |_, this, ()| {
            Ok(this.java_string_check_utf8())
        });

        methods.add_method("has_java_string_check_utf8", |_, this, ()| {
            Ok(this.has_java_string_check_utf8())
        });

        methods.add_method_mut("clear_java_string_check_utf8", |_, this, ()| {
            this.clear_java_string_check_utf8();
            Ok(())
        });

        methods.add_method_mut("set_java_string_check_utf8", |_, this, v: bool| {
            this.set_java_string_check_utf8(v);
            Ok(())
        });

        methods.add_method("optimize_for", |_, this, ()| {
            Ok(this.optimize_for() as i32)
        });

        methods.add_method("has_optimize_for", |_, this, ()| {
            Ok(this.has_optimize_for())
        });

        methods.add_method_mut("clear_optimize_for", |_, this, ()| {
            this.clear_optimize_for();
            Ok(())
        });

        methods.add_method_mut("set_optimize_for", |_, this, v: i32| {
            let v = OptimizeMode::from_i32(v).ok_or(anyhow!("unknown optimization mode {v}"))?;
            this.set_optimize_for(v);
            Ok(())
        });

        methods.add_method("go_package", |_, this, ()| {
            Ok(this.go_package().to_string())
        });

        methods.add_method("has_go_package", |_, this, ()| {
            Ok(this.has_go_package())
        });

        methods.add_method_mut("clear_go_package", |_, this, ()| {
            this.clear_go_package();
            Ok(())
        });

        methods.add_method_mut("set_go_package", |_, this, v: String| {
            this.set_go_package(v);
            Ok(())
        });

        methods.add_method_mut("mut_go_package", |_, this, ()| {
            Ok(this.mut_go_package().clone())
        });

        methods.add_method_mut("take_go_package", |_, this, ()| {
            Ok(this.take_go_package())
        });

        methods.add_method("cc_generic_services", |_, this, ()| {
            Ok(this.cc_generic_services())
        });

        methods.add_method("has_cc_generic_services", |_, this, ()| {
            Ok(this.has_cc_generic_services())
        });

        methods.add_method_mut("clear_cc_generic_services", |_, this, ()| {
            this.clear_cc_generic_services();
            Ok(())
        });

        methods.add_method_mut("set_cc_generic_services", |_, this, v: bool| {
            this.set_cc_generic_services(v);
            Ok(())
        });

        methods.add_method("java_generic_services", |_, this, ()| {
            Ok(this.java_generic_services())
        });

        methods.add_method("has_java_generic_services", |_, this, ()| {
            Ok(this.has_java_generic_services())
        });

        methods.add_method_mut("clear_java_generic_services", |_, this, ()| {
            this.clear_java_generic_services();
            Ok(())
        });

        methods.add_method_mut("set_java_generic_services", |_, this, v: bool| {
            this.set_java_generic_services(v);
            Ok(())
        });

        methods.add_method("py_generic_services", |_, this, ()| {
            Ok(this.py_generic_services())
        });

        methods.add_method("has_py_generic_services", |_, this, ()| {
            Ok(this.has_py_generic_services())
        });

        methods.add_method_mut("clear_py_generic_services", |_, this, ()| {
            this.clear_py_generic_services();
            Ok(())
        });

        methods.add_method_mut("set_py_generic_services", |_, this, v: bool| {
            this.set_py_generic_services(v);
            Ok(())
        });

        methods.add_method("php_generic_services", |_, this, ()| {
            Ok(this.php_generic_services())
        });

        methods.add_method("has_php_generic_services", |_, this, ()| {
            Ok(this.has_php_generic_services())
        });

        methods.add_method_mut("clear_php_generic_services", |_, this, ()| {
            this.clear_php_generic_services();
            Ok(())
        });

        methods.add_method_mut("set_php_generic_services", |_, this, v: bool| {
            this.set_php_generic_services(v);
            Ok(())
        });

        methods.add_method("deprecated", |_, this, ()| {
            Ok(this.deprecated())
        });

        methods.add_method("has_deprecated", |_, this, ()| {
            Ok(this.has_deprecated())
        });

        methods.add_method_mut("clear_deprecated", |_, this, ()| {
            this.clear_deprecated();
            Ok(())
        });

        methods.add_method_mut("set_deprecated", |_, this, v: bool| {
            this.set_deprecated(v);
            Ok(())
        });

        methods.add_method("cc_enable_arenas", |_, this, ()| {
            Ok(this.cc_enable_arenas())
        });

        methods.add_method("has_cc_enable_arenas", |_, this, ()| {
            Ok(this.has_cc_enable_arenas())
        });

        methods.add_method_mut("clear_cc_enable_arenas", |_, this, ()| {
            this.clear_cc_enable_arenas();
            Ok(())
        });

        methods.add_method_mut("set_cc_enable_arenas", |_, this, v: bool| {
            this.set_cc_enable_arenas(v);
            Ok(())
        });

        methods.add_method("objc_class_prefix", |_, this, ()| {
            Ok(this.objc_class_prefix().to_string())
        });

        methods.add_method("has_objc_class_prefix", |_, this, ()| {
            Ok(this.has_objc_class_prefix())
        });

        methods.add_method_mut("clear_objc_class_prefix", |_, this, ()| {
            this.clear_objc_class_prefix();
            Ok(())
        });

        methods.add_method_mut("set_objc_class_prefix", |_, this, v: String| {
            this.set_objc_class_prefix(v);
            Ok(())
        });

        methods.add_method_mut("mut_objc_class_prefix", |_, this, ()| {
            Ok(this.mut_objc_class_prefix().clone())
        });

        methods.add_method_mut("take_objc_class_prefix", |_, this, ()| {
            Ok(this.take_objc_class_prefix())
        });

        methods.add_method("csharp_namespace", |_, this, ()| {
            Ok(this.csharp_namespace().to_string())
        });

        methods.add_method("has_csharp_namespace", |_, this, ()| {
            Ok(this.has_csharp_namespace())
        });

        methods.add_method_mut("clear_csharp_namespace", |_, this, ()| {
            this.clear_csharp_namespace();
            Ok(())
        });

        methods.add_method_mut("set_csharp_namespace", |_, this, v: String| {
            this.set_csharp_namespace(v);
            Ok(())
        });

        methods.add_method_mut("mut_csharp_namespace", |_, this, ()| {
            Ok(this.mut_csharp_namespace().clone())
        });

        methods.add_method_mut("take_csharp_namespace", |_, this, ()| {
            Ok(this.take_csharp_namespace())
        });

        methods.add_method("swift_prefix", |_, this, ()| {
            Ok(this.swift_prefix().to_string())
        });

        methods.add_method("has_swift_prefix", |_, this, ()| {
            Ok(this.has_swift_prefix())
        });

        methods.add_method_mut("clear_swift_prefix", |_, this, ()| {
            this.clear_swift_prefix();
            Ok(())
        });

        methods.add_method_mut("set_swift_prefix", |_, this, v: String| {
            this.set_swift_prefix(v);
            Ok(())
        });

        methods.add_method_mut("mut_swift_prefix", |_, this, ()| {
            Ok(this.mut_swift_prefix().clone())
        });

        methods.add_method_mut("take_swift_prefix", |_, this, ()| {
            Ok(this.take_swift_prefix())
        });

        methods.add_method("php_class_prefix", |_, this, ()| {
            Ok(this.php_class_prefix().to_string())
        });

        methods.add_method("has_php_class_prefix", |_, this, ()| {
            Ok(this.has_php_class_prefix())
        });

        methods.add_method_mut("clear_php_class_prefix", |_, this, ()| {
            this.clear_php_class_prefix();
            Ok(())
        });

        methods.add_method_mut("set_php_class_prefix", |_, this, v: String| {
            this.set_php_class_prefix(v);
            Ok(())
        });

        methods.add_method_mut("mut_php_class_prefix", |_, this, ()| {
            Ok(this.mut_php_class_prefix().clone())
        });

        methods.add_method_mut("take_php_class_prefix", |_, this, ()| {
            Ok(this.take_php_class_prefix())
        });

        methods.add_method("php_namespace", |_, this, ()| {
            Ok(this.php_namespace().to_string())
        });

        methods.add_method("has_php_namespace", |_, this, ()| {
            Ok(this.has_php_namespace())
        });

        methods.add_method_mut("clear_php_namespace", |_, this, ()| {
            this.clear_php_namespace();
            Ok(())
        });

        methods.add_method_mut("set_php_namespace", |_, this, v: String| {
            this.set_php_namespace(v);
            Ok(())
        });

        methods.add_method_mut("mut_php_namespace", |_, this, ()| {
            Ok(this.mut_php_namespace().clone())
        });

        methods.add_method_mut("take_php_namespace", |_, this, ()| {
            Ok(this.take_php_namespace())
        });

        methods.add_method("php_metadata_namespace", |_, this, ()| {
            Ok(this.php_metadata_namespace().to_string())
        });

        methods.add_method("has_php_metadata_namespace", |_, this, ()| {
            Ok(this.has_php_metadata_namespace())
        });

        methods.add_method_mut("clear_php_metadata_namespace", |_, this, ()| {
            this.clear_php_metadata_namespace();
            Ok(())
        });

        methods.add_method_mut("set_php_metadata_namespace", |_, this, v: String| {
            this.set_php_metadata_namespace(v);
            Ok(())
        });

        methods.add_method_mut("mut_php_metadata_namespace", |_, this, ()| {
            Ok(this.mut_php_metadata_namespace().clone())
        });

        methods.add_method_mut("take_php_metadata_namespace", |_, this, ()| {
            Ok(this.take_php_metadata_namespace())
        });

        methods.add_method("ruby_package", |_, this, ()| {
            Ok(this.ruby_package().to_string())
        });

        methods.add_method("has_ruby_package", |_, this, ()| {
            Ok(this.has_ruby_package())
        });

        methods.add_method_mut("clear_ruby_package", |_, this, ()| {
            this.clear_ruby_package();
            Ok(())
        });

        methods.add_method_mut("set_ruby_package", |_, this, v: String| {
            this.set_ruby_package(v);
            Ok(())
        });

        methods.add_method_mut("mut_ruby_package", |_, this, ()| {
            Ok(this.mut_ruby_package().clone())
        });

        methods.add_method_mut("take_ruby_package", |_, this, ()| {
            Ok(this.take_ruby_package())
        });

        add_message_trait_method!(methods, FileOptions, LuaFileOptions);

        add_message_dyn_trait_method!(methods, FileOptions, LuaFileOptions);

        add_message_full_trait_method!(methods, FileOptions, LuaFileOptions);
    }
}