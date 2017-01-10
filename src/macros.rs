macro_rules! primitive_enum {
    ( 
        pub enum $name:ident : $primitivetype:ty : $repr:ident {
            $($member:ident = $value:expr),* 
        }
    ) 
    => 
    {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr($repr)]
        pub enum $name {
            $($member = $value,)*
        }
        
        impl $name {
            #[inline]
            pub fn from_value(value: $primitivetype) -> Option<$name> {
                Some(match value {
                    $($value => $name::$member,)*
                    _ => return None,
                })
            }
            
            #[inline]
            pub fn value(&self) -> $primitivetype {
                match *self {
                    $($name::$member => $value),*
                }
            }
        }
    }
}