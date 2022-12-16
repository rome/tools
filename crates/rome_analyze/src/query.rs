use std::any::TypeId;

use rome_rowan::{Language, SyntaxKindSet, TextRange};

use crate::{registry::Phase, services::FromServices, Phases, ServiceBag, Visitor};

/// Trait implemented for all types, for example lint rules can query them to emit diagnostics or code actions.
pub trait Queryable: Sized {
    type Input: QueryMatch;
    type Output;

    type Language: Language;
    type Services: FromServices + Phase;

    fn build_visitor(
        analyzer: &mut impl AddVisitor<Self::Language>,
        root: &<Self::Language as Language>::Root,
    );

    fn key() -> QueryKey<Self::Language> {
        QueryKey::TypeId(TypeId::of::<Self::Input>())
    }

    /// Unwrap an instance of `Self` from a [QueryMatch].
    ///
    /// ## Panics
    ///
    /// If the [QueryMatch] variant of `query` doesn't match `Self::KEY`
    fn unwrap_match(services: &ServiceBag, query: &Self::Input) -> Self::Output;
}

pub trait AddVisitor<L: Language> {
    fn add_visitor<F, V>(&mut self, phase: Phases, visitor: F)
    where
        F: FnOnce() -> V,
        V: Visitor<Language = L> + 'static;
}

/// Marker trait implemented for all the types analyzer visitors may emit
pub trait QueryMatch: 'static {
    fn text_range(&self) -> TextRange;
}

/// Represents which type a given [Queryable] type can match, either a specific
/// subset of syntax node kinds or any generic type
pub enum QueryKey<L: Language> {
    Syntax(SyntaxKindSet<L>),
    TypeId(TypeId),
}
