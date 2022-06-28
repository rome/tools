use rome_rowan::{AstNode, Language, SyntaxKindSet, SyntaxNode};

use crate::registry::NodeLanguage;

/// Trait implemented for all types, for example lint rules can query them to emit diagnostics or code actions.
pub trait Queryable: Sized {
    type Language: Language;

    const KEY: QueryKey<Self::Language>;

    /// Unwrap an instance of `Self` from a [QueryMatch].
    ///
    /// ## Panics
    ///
    /// If the type is mismatched
    fn unwrap_match(query: &QueryMatch<Self::Language>) -> Self;
}

pub enum QueryKey<L: Language> {
    Syntax(SyntaxKindSet<L>),
}

pub enum QueryMatch<L: Language> {
    Syntax(SyntaxNode<L>),
}

/// Query type usable by lint rules to match on specific [AstNode] types
#[derive(Clone)]
pub struct Ast<N>(pub N);

impl<N> Queryable for Ast<N>
where
    N: AstNode + 'static,
{
    type Language = NodeLanguage<N>;

    const KEY: QueryKey<Self::Language> = QueryKey::Syntax(N::KIND_SET);

    fn unwrap_match(query: &QueryMatch<Self::Language>) -> Self {
        match query {
            QueryMatch::Syntax(node) => Self(N::unwrap_cast(node.clone())),
        }
    }
}
