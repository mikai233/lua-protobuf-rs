#[macro_export]
macro_rules! add_message_trait_method {
    ($methods:ident, $target:ty, $wrapper:ty) => {
        $methods.add_method("is_initialized", |_, this, ()| { 
            Ok(protobuf::Message::is_initialized(&**this))
        });
        
        $methods.add_method("compute_size", |_, this, ()| { 
            Ok(protobuf::Message::compute_size(&**this))
        });
        
        $methods.add_method("cached_size", |_, this, ()| { 
            Ok(protobuf::Message::cached_size(&**this)) 
        });
        
        $methods.add_method("write_length_delimited_to_vec", |_, this, ()| { 
            let mut vec = Vec::new();
            protobuf::Message::write_length_delimited_to_vec(&**this, &mut vec).map_err(|e|anyhow::anyhow!(e))?;
            Ok(vec) 
        });
        
        $methods.add_method_mut("merge_from_bytes", |_, this, bytes: Vec<u8>| { 
            protobuf::Message::merge_from_bytes(&mut **this, &bytes).map_err(|e|anyhow::anyhow!(e))?;
            Ok(()) 
        });
        
        $methods.add_function("parse_from_bytes", |_, bytes: Vec<u8>| { 
            let myself: $target = <$target as protobuf::Message>::parse_from_bytes(&bytes).map_err(|e|anyhow::anyhow!(e))?;
            let myself: $wrapper = myself.into();
            Ok(myself) 
        });
        
        $methods.add_method("check_initialized", |_, this, ()| {
            Ok(protobuf::Message::check_initialized(&**this).map_err(|e|anyhow::anyhow!(e))?)
        });
        
        $methods.add_method("write_to_vec", |_, this, ()| { 
            let mut vec = Vec::new();
            protobuf::Message::write_to_vec(&**this, &mut vec).map_err(|e|anyhow::anyhow!(e))?;
            Ok(vec) 
        });
        
        $methods.add_method("write_to_bytes", |_, this, ()| { 
            let bytes = protobuf::Message::write_to_bytes(&**this).map_err(|e|anyhow::anyhow!(e))?;
            Ok(bytes) 
        });
        
        $methods.add_method("write_length_delimited_to_bytes", |_, this, ()| { 
            let bytes = protobuf::Message::write_length_delimited_to_bytes(&**this).map_err(|e|anyhow::anyhow!(e))?;
            Ok(bytes) 
        });
        
        $methods.add_function("new", |_, ()| { 
            let myself: $target = <$target as protobuf::Message>::new();
            let myself: $wrapper = myself.into();
            Ok(myself) 
        });
        
        $methods.add_method_mut("clear", |_, this, ()| { 
            Ok(protobuf::Message::clear(&mut **this))
        });
        
        $methods.add_function("default_instance", |_, ()| { 
            let myself = <$target as protobuf::Message>::default_instance().clone();
            let myself: $wrapper = myself.into();
            Ok(myself)
        });
    };
}