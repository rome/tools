use std::collections::HashSet;

use rome_diagnostics::file::FileId;
use rome_rowan::{AstNode, Language, SyntaxKind, SyntaxNode};

use crate::{
    context::RuleContext,
    signals::{AnalyzerSignal, RuleSignal},
    ControlFlow, Rule,
};

/// The rule registry holds type-erased instances of all active analysis rules
pub struct RuleRegistry<L: Language> {
    /// Holds a collection of rules for each [SyntaxKind] node type that has
    /// lint rules associated with it
    nodes: Vec<KindRules<L>>,
}

impl<L: Language> RuleRegistry<L> {
    pub fn empty() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn push<R>(&mut self)
    where
        R: Rule + 'static,
        R::Query: AstNode<Language = L>,
    {
        for kind in <R::Query as AstNode>::KIND_SET.iter() {
            let index = usize::from(kind.to_raw().0);

            if self.nodes.len() <= index {
                self.nodes.resize_with(index + 1, KindRules::empty);
            }

            let node = &mut self.nodes[index];
            node.rules.push(RegistryRule::of::<R>());
        }
    }

    /// Returns an iterator over the name and documentation of all active rules
    /// in this instance of the registry
    pub fn metadata(self) -> impl Iterator<Item = (&'static str, &'static str)> {
        let mut unique = HashSet::new();
        self.nodes
            .into_iter()
            .flat_map(|node| node.rules)
            .map(|rule| (rule.name, rule.docs))
            .filter(move |(name, _)| unique.insert(name.as_ptr() as u64))
    }
}

/// [KindRules] holds a collection of [Rule]s that match a specific [SyntaxKind] value
struct KindRules<L: Language> {
    rules: Vec<RegistryRule<L>>,
}

impl<L: Language> KindRules<L> {
    fn empty() -> Self {
        Self { rules: Vec::new() }
    }
}

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
        let kind = usize::from(node.kind().to_raw().0);
        let rules = match self.nodes.get(kind) {
            Some(entry) => &entry.rules,
            None => return ControlFlow::Continue(()),
        };
        for rule in rules {
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
            // SAFETY: The rule should never get executed in the first place
            // if the query doesn't match
            let query_result = <R::Query>::unwrap_cast(node.clone());
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
