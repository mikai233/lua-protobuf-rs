#[macro_export]
macro_rules! add_message_dyn_trait_method {
    ($methods:ident, $target:ty, $wrapper:ty) => {
        $methods.add_method("descriptor_dyn", |_, this, ()| {
            Ok::<$crate::descriptor::message_descriptor::LuaMessageDescriptor, _>(
                protobuf::MessageDyn::descriptor_dyn(&**this).into(),
            )
        });

        $methods.add_method("compute_size_dyn", |_, this, ()| {
            Ok(protobuf::MessageDyn::compute_size_dyn(&**this))
        });

        $methods.add_method("is_initialized_dyn", |_, this, ()| {
            Ok(protobuf::MessageDyn::is_initialized_dyn(&**this))
        });
    };
}
