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
            const PROPS: &[(&'static str, bool)] = $p_value;
            const ROLES: &[&'static str] = $r_value;
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
