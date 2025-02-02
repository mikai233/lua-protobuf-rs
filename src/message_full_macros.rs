#[macro_export]
macro_rules! add_message_full_trait_method {
    ($methods:ident, $target:ty, $wrapper:ty) => {
        $methods.add_function("descriptor", |_, ()| {
            let descriptor: crate::descriptor::message_descriptor::LuaMessageDescriptor = <$target as protobuf::MessageFull>::descriptor().into();
            Ok(descriptor)
        });
    }
}