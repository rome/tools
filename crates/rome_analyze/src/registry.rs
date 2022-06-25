use std::{iter::Map, vec::IntoIter};

use rome_diagnostics::file::FileId;
use rome_rowan::{AstNode, Language, SyntaxNode};

use crate::{
    context::RuleContext,
    signals::{AnalyzerSignal, RuleSignal},
    ControlFlow, Rule,
};

/// The rule registry holds type-erased instances of all active analysis rules
pub struct RuleRegistry<L: Language> {
    rules: Vec<RegistryRule<L>>,
}

impl<L: Language> RuleRegistry<L> {
    pub fn empty() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn push<R>(&mut self)
    where
        R: Rule + 'static,
        R::Query: AstNode<Language = L>,
    {
        self.rules.push(RegistryRule::of::<R>());
    }

    /// Returns an iterator over the name and documentation of all active rules
    /// in this instance of the registry
    pub fn metadata(self) -> MetadataIter<L> {
        self.rules.into_iter().map(|rule| (rule.name, rule.docs))
    }
}

pub type MetadataIter<L> =
    Map<IntoIter<RegistryRule<L>>, fn(RegistryRule<L>) -> (&'static str, &'static str)>;

pub(crate) type RuleLanguage<R> = NodeLanguage<<R as Rule>::Query>;
pub(crate) type NodeLanguage<N> = <N as AstNode>::Language;

pub(crate) type RuleRoot<R> = LanguageRoot<RuleLanguage<R>>;
pub type LanguageRoot<L> = <L as Language>::Root;

impl<L> RuleRegistry<L>
where
    L: Language,
{
    // Run all rules known to the registry associated with nodes of type N
    pub(crate) fn analyze<B>(
        &self,
        file_id: FileId,
        root: &LanguageRoot<L>,
        node: SyntaxNode<L>,
        callback: &mut impl FnMut(&dyn AnalyzerSignal<L>) -> ControlFlow<B>,
    ) -> ControlFlow<B> {
        for rule in &self.rules {
            if let Some(event) = (rule.run)(file_id, root, &node) {
                if let ControlFlow::Break(b) = callback(&*event) {
                    return ControlFlow::Break(b);
                }
            }
        }

        ControlFlow::Continue(())
    }
}

/// Executor for rule as a generic function pointer
type RuleExecutor<L> = for<'a> fn(
    FileId,
    &'a LanguageRoot<L>,
    &'a SyntaxNode<L>,
) -> Option<Box<dyn AnalyzerSignal<L> + 'a>>;

#[doc(hidden)]
/// Internal representation of a single rule in the registry
pub struct RegistryRule<L: Language> {
    name: &'static str,
    docs: &'static str,
    run: RuleExecutor<L>,
}

impl<L: Language> RegistryRule<L> {
    const fn of<R>() -> Self
    where
        R: Rule + 'static,
        R::Query: AstNode<Language = L>,
    {
        /// Generic implementation of RuleExecutor for any rule type R
        fn run<'a, R: Rule + 'static>(
            file_id: FileId,
            root: &'a RuleRoot<R>,
            node: &'a SyntaxNode<<R::Query as AstNode>::Language>,
        ) -> Option<Box<dyn AnalyzerSignal<RuleLanguage<R>> + 'a>> {
            if !<R::Query>::can_cast(node.kind()) {
                return None;
            }

            let query_result = <R::Query>::cast(node.clone())?;
            let ctx = RuleContext::new(query_result.clone(), root.clone());

            let result = R::run(&ctx)?;
            Some(RuleSignal::<R>::new_boxed(
                file_id,
                root,
                query_result,
                result,
            ))
        }

        Self {
            name: R::NAME,
            docs: R::DOCS,
            run: run::<R>,
        }
    }
}
