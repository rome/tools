#[macro_export]
macro_rules! define_role {
    ( $( #[doc = $doc:literal] )+ $id:ident {
        PROPS: $p_value:expr,
        ROLES: $r_value:expr,
    }) => {
        $( #[doc = $doc] )*
        #[derive(Debug)]
        struct $id;

        impl $id {
            const PROPS: &[(&'static str, bool)] = &$p_value;
            const ROLES: &[&'static str] = &$r_value;
        }

        impl $crate::AriaRoleDefinition for $id {
            fn properties<'a>(&self) -> std::slice::Iter<'a, (&str, bool)> {
                $id::PROPS.iter()
            }

            fn roles<'a>(&self) -> std::slice::Iter<'a, &str> {
                $id::ROLES.iter()
            }
        }
    };
}

#[macro_export]
macro_rules! define_property {
    ( $id:ident {
        PROPERTY_TYPE: $property_type:literal,
        VALUES: $values:expr,
    }) => {
        #[derive(Debug)]
        struct $id;

        impl $id {
            const PROPERTY_TYPE: &'static str = &$property_type;
            const VALUES: &[&'static str] = &$values;
        }

        impl AriaPropertyDefinition for $id {
            fn values<'a>(&self) -> std::slice::Iter<'a, &str> {
                $id::VALUES.iter()
            }

            fn property_type<'a>(&self) -> $crate::AriaPropertyTypeEnum {
                // SAFETY: PROPERTY_TYPE is internal and should not contain extraneous properties
                $crate::AriaPropertyTypeEnum::from_str($id::PROPERTY_TYPE).unwrap()
            }
        }
    };
}
