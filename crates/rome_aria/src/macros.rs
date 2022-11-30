#[macro_export]
macro_rules! define_role {
    ($id:ident {
        PROPS: [$p_count:literal, $p_value:expr ],
        ROLES: [$r_count:literal, $r_value:expr ],
    }) => {
        #[derive(Debug)]
        struct $id;

        impl $id {
            const PROPS: [(&'static str, bool); $p_count] = $p_value;
            const ROLES: [&'static str; $r_count] = $r_value;
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
