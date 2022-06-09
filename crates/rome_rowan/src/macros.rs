use crate::{AstNode, Language};

/// Declares a custom union AstNode type with an ungram-like syntax
///
/// # Example
///
/// ```ignore
/// declare_union! {
///     /// Matches an if statement or a conditional expression
///     pub(crate) JsAnyConditional = JsIfStatement | JsConditionalExpression
/// }
/// ```
#[macro_export]
macro_rules! declare_union {
    ( $( #[$attr:meta] )* $vis:vis $name:ident = $( $variant:ident )|* ) => {
        $( #[$attr] )*
        #[allow(clippy::enum_variant_names)]
        #[derive(Clone, PartialEq, Eq, Hash)]
        $vis enum $name {
            $( $variant($variant), )*
        }

        impl AstNode for $name {
            type Language = <( $( $variant, )* ) as $crate::macros::UnionLanguage>::Language;

            fn can_cast(kind: <Self::Language as $crate::Language>::Kind) -> bool {
                $( $variant::can_cast(kind) )||*
            }

            fn cast(syntax: $crate::SyntaxNode<Self::Language>) -> Option<Self>
            where
                Self: Sized,
            {
                $( if $variant::can_cast(syntax.kind()) {
                    return Some(Self::$variant($variant::unwrap_cast(syntax)));
                } )*

                None
            }

            fn syntax(&self) -> &$crate::SyntaxNode<Self::Language> {
                match self {
                    $( Self::$variant(node) => node.syntax() ),*
                }
            }

            fn into_syntax(self) -> $crate::SyntaxNode<Self::Language> {
                match self {
                    $( Self::$variant(node) => node.into_syntax() ),*
                }
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $( Self::$variant(it) => std::fmt::Debug::fmt(it, f), )*
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Display::fmt(self.syntax(), f)
            }
        }

        $( impl From<$variant> for $name {
            fn from(node: $variant) -> Self {
                Self::$variant(node)
            }
        } )*

        impl From<$name> for $crate::SyntaxNode<<$name as AstNode>::Language> {
            fn from(n: $name) -> $crate::SyntaxNode<<$name as AstNode>::Language> {
                match n {
                    $( $name::$variant(it) => it.into(), )*
                }
            }
        }

        impl From<$name> for $crate::SyntaxElement<<$name as AstNode>::Language> {
            fn from(n: $name) -> $crate::SyntaxElement<<$name as AstNode>::Language> {
                $crate::SyntaxNode::<<$name as AstNode>::Language>::from(n).into()
            }
        }
    };
}

/// This trait is implemented for tuples of AstNode types of size 1 to 9 if all
/// node types share the same associated language (which is then aliased as the
/// `Language` associated type on [UnionLanguage] itself)
pub trait UnionLanguage {
    type Language: Language;
}

macro_rules! impl_union_language {
    ( $head:ident $( , $rest:ident )* ) => {
        impl<$head $( , $rest )*> UnionLanguage for ($head, $( $rest ),*)
        where
            $head: AstNode $( , $rest: AstNode<Language = <$head as AstNode>::Language> )*
        {
            type Language = <$head as AstNode>::Language;
        }
    };
}

impl_union_language!(T1);
impl_union_language!(T1, T2);
impl_union_language!(T1, T2, T3);
impl_union_language!(T1, T2, T3, T4);
impl_union_language!(T1, T2, T3, T4, T5);
impl_union_language!(T1, T2, T3, T4, T5, T6);
impl_union_language!(T1, T2, T3, T4, T5, T6, T7);
impl_union_language!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_union_language!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
