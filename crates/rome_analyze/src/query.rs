use rome_control_flow::ControlFlowGraph;
use rome_rowan::{AstNode, Language, SyntaxKindSet, SyntaxNode, TextRange};

use crate::{
    registry::{NodeLanguage, Phase},
    services::FromServices,
    Phases, ServiceBag, SyntaxVisitor, Visitor,
};

/// Trait implemented for all types, for example lint rules can query them to emit diagnostics or code actions.
pub trait Queryable: Sized {
    type Output;
    type Language: Language;
    type Services: FromServices + Phase;

    fn build_visitor(
        analyzer: &mut impl AddVisitor<Self::Language>,
        root: &<Self::Language as Language>::Root,
    );

    /// Statically declares which [QueryMatch] variant is matched by this
    /// [Queryable] type. For instance the [Ast] queryable matches on
    /// [QueryMatch::Syntax], so its key is defined as [QueryKey::Syntax]
    const KEY: QueryKey<Self::Language>;

    /// Unwrap an instance of `Self` from a [QueryMatch].
    ///
    /// ## Panics
    ///
    /// If the [QueryMatch] variant of `query` doesn't match `Self::KEY`
    fn unwrap_match(services: &ServiceBag, query: &QueryMatch<Self::Language>) -> Self::Output;
}

pub trait AddVisitor<L: Language> {
    fn add_visitor<V>(&mut self, phase: Phases, visitor: V)
    where
        V: Visitor<Language = L> + 'static;
}

/// Enumerate all the types of [Queryable] analyzer visitors may emit
#[derive(Clone, Debug)]
pub enum QueryMatch<L: Language> {
    Syntax(SyntaxNode<L>),
    SemanticModel(TextRange),
    ControlFlowGraph(ControlFlowGraph<L>, TextRange),
}

impl<L: Language> QueryMatch<L> {
    pub fn text_range(&self) -> TextRange {
        match self {
            QueryMatch::Syntax(node) => node.text_trimmed_range(),
            QueryMatch::SemanticModel(range) | QueryMatch::ControlFlowGraph(_, range) => *range,
        }
    }
}

/// Mirrors the variants of [QueryMatch] to statically compute which queries a
/// given [Queryable] type can match
pub enum QueryKey<L: Language> {
    Syntax(SyntaxKindSet<L>),
    ControlFlowGraph,
    SemanticModel,
}

/// Query type usable by lint rules to match on specific [AstNode] types
#[derive(Clone)]
pub struct Ast<N>(pub N);

impl<N> Queryable for Ast<N>
where
    N: AstNode + 'static,
{
    type Output = N;
    type Language = NodeLanguage<N>;
    type Services = ();

    fn build_visitor(
        analyzer: &mut impl AddVisitor<Self::Language>,
        _: &<Self::Language as Language>::Root,
    ) {
        analyzer.add_visitor(Phases::Syntax, SyntaxVisitor::default());
    }

    /// Match on [QueryMatch::Syntax] if the kind of the syntax node matches
    /// the kind set of `N`
    const KEY: QueryKey<Self::Language> = QueryKey::Syntax(N::KIND_SET);

    fn unwrap_match(_: &ServiceBag, query: &QueryMatch<Self::Language>) -> Self::Output {
        match query {
            QueryMatch::Syntax(node) => N::unwrap_cast(node.clone()),
            _ => panic!("tried to unwrap unsupported QueryMatch kind, expected Syntax"),
        }
    }
}
