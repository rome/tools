#[macro_export]
macro_rules! declare_transformation {
    ( $( #[doc = $doc:literal] )+ $vis:vis $id:ident {
        version: $version:literal,
        name: $name:tt,
        $( $key:ident: $value:expr, )*
    } ) => {
        $( #[doc = $doc] )*
        $vis enum $id {}

        impl ::rome_analyze::RuleMeta for $id {
            type Group = $crate::registry::TransformationGroup;
            const METADATA: ::rome_analyze::RuleMetadata =
                ::rome_analyze::RuleMetadata::new($version, $name, concat!( $( $doc, "\n", )* )) $( .$key($value) )*;
        }
    };
}
