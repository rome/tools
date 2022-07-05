use std::collections::HashSet;

use rome_diagnostics::file::FileId;
use rome_rowan::{AstNode, Language, RawSyntaxKind, SyntaxKind};

use crate::{
    context::RuleContext,
    query::{QueryKey, QueryMatch, Queryable},
    services::ServiceBag,
    signals::{AnalyzerSignal, RuleSignal},
    ControlFlow, Rule,
};

#[repr(usize)]
pub enum Phases {
    Syntax = 0,
    Semantic = 1,
}

pub trait Phase {
    fn phase() -> Phases {
        Phases::Syntax
    }
}

impl Phase for () {}

/// The rule registry holds type-erased instances of all active analysis rules
pub struct RuleRegistry<'a, L: Language, B> {
    /// Holds a collection of rules for each [SyntaxKind] node type that has
    /// lint rules associated with it for each phase
    phases: [Vec<SyntaxKindRules<L, B>>; 2],
    control_flow: Vec<RegistryRule<L, B>>,
    emit_signal: Box<dyn FnMut(&dyn AnalyzerSignal<L>) -> ControlFlow<B> + 'a>,
}

impl<'a, L: Language, B> RuleRegistry<'a, L, B> {
    pub fn new(emit_signal: impl FnMut(&dyn AnalyzerSignal<L>) -> ControlFlow<B> + 'a) -> Self {
        Self {
            phases: [Vec::new(), Vec::new()],
            control_flow: Vec::new(),
            emit_signal: Box::new(emit_signal),
        }
    }

    /// Add the rule `R` to the list of rules stores in this registry instance
    pub fn push<R>(&mut self)
    where
        R: Rule + 'static,
        R::Query: Queryable<Language = L> + Clone,
    {
        let phase = R::phase() as usize;

        match <R::Query as Queryable>::KEY {
            QueryKey::Syntax(key) => {
                // Iterate on all the SyntaxKind variants this node can match
                for kind in key.iter() {
                    // Convert the numerical value of `kind` to an index in the
                    // `nodes` vector
                    let RawSyntaxKind(index) = kind.to_raw();
                    let index = usize::from(index);

                    // Ensure the vector has enough capacity by inserting empty
                    // `SyntaxKindRules` as required
                    if self.phases[phase].len() <= index {
                        self.phases[phase].resize_with(index + 1, SyntaxKindRules::new);
                    }

                    // Insert a handle to the rule `R` into the `SyntaxKindRules` entry
                    // corresponding to the SyntaxKind index
                    let node = &mut self.phases[phase][index];
                    node.rules.push(RegistryRule::of::<R>());
                }
            }
            QueryKey::ControlFlowGraph => {
                self.control_flow.push(RegistryRule::of::<R>());
            }
        }
    }

    /// Returns an iterator over the name and documentation of all active rules
    /// in this instance of the registry
    pub fn metadata(&self) -> Vec<(&'static str, &'static str)> {
        let mut unique = HashSet::new();

        let [phase0, phase1] = &self.phases;

        let mut m: Vec<_> = phase0
            .iter()
            .flat_map(|node| node.rules.iter())
            .map(|rule| (rule.name, rule.docs))
            .filter(|(name, _)| unique.insert(name.as_ptr() as u64))
            .collect();

        m.extend(
            phase1
                .iter()
                .flat_map(|node| node.rules.iter())
                .map(|rule| (rule.name, rule.docs))
                .filter(|(name, _)| unique.insert(name.as_ptr() as u64)),
        );

        m
    }
}

/// [SyntaxKindRules] holds a collection of [Rule]s that match a specific [SyntaxKind] value
struct SyntaxKindRules<L: Language, B> {
    rules: Vec<RegistryRule<L, B>>,
}

impl<L: Language, B> SyntaxKindRules<L, B> {
    fn new() -> Self {
        Self { rules: Vec::new() }
    }
}

pub(crate) type RuleLanguage<R> = QueryLanguage<<R as Rule>::Query>;
pub(crate) type QueryLanguage<N> = <N as Queryable>::Language;
pub(crate) type NodeLanguage<N> = <N as AstNode>::Language;

pub(crate) type RuleRoot<R> = LanguageRoot<RuleLanguage<R>>;
pub type LanguageRoot<L> = <L as Language>::Root;

impl<'a, L, B> RuleRegistry<'a, L, B>
where
    L: Language,
{
    // Run all rules known to the registry associated with nodes of type N
    pub fn match_query(
        &mut self,
        phase: Phases,
        file_id: FileId,
        root: &LanguageRoot<L>,
        query: &QueryMatch<L>,
        services: &ServiceBag,
    ) -> ControlFlow<B> {
        let phase = phase as usize;

        let rules = match query {
            QueryMatch::Syntax(node) => {
                // Convert the numerical value of the SyntaxKind to an index in the
                // `syntax` vector
                let RawSyntaxKind(kind) = node.kind().to_raw();
                let kind = usize::from(kind);

                // Lookup the syntax entry corresponding to the SyntaxKind index
                match self.phases[phase].get(kind) {
                    Some(entry) => &entry.rules,
                    None => return ControlFlow::Continue(()),
                }
            }
            QueryMatch::ControlFlowGraph(_) => &self.control_flow,
        };

        // Run all the rules registered to this QueryMatch
        for rule in rules {
            (rule.run)(file_id, root, query, services, &mut self.emit_signal)?;
        }

        ControlFlow::Continue(())
    }
}

/// Executor for rule as a generic function pointer
type RuleExecutor<L, B> = for<'a> fn(
    FileId,
    &'a LanguageRoot<L>,
    &'a QueryMatch<L>,
    &'a ServiceBag,
    &'a mut dyn FnMut(&dyn AnalyzerSignal<L>) -> ControlFlow<B>,
) -> ControlFlow<B>;

/// Internal representation of a single rule in the registry
pub struct RegistryRule<L: Language, B> {
    name: &'static str,
    docs: &'static str,
    run: RuleExecutor<L, B>,
}

impl<L: Language, B> RegistryRule<L, B> {
    fn of<R>() -> Self
    where
        R: Rule + 'static,
        R::Query: Queryable<Language = L> + Clone + 'static,
    {
        /// Generic implementation of RuleExecutor for any rule type R
        fn run<'a, R, B>(
            file_id: FileId,
            root: &'a RuleRoot<R>,
            query: &'a QueryMatch<RuleLanguage<R>>,
            services: &'a ServiceBag,
            callback: &'a mut dyn FnMut(&dyn AnalyzerSignal<RuleLanguage<R>>) -> ControlFlow<B>,
        ) -> ControlFlow<B>
        where
            R: Rule + 'static,
            R::Query: Clone + 'static,
        {
            // SAFETY: The rule should never get executed in the first place
            // if the query doesn't match
            let query_result = <R::Query as Queryable>::unwrap_match(query);
            let ctx = match RuleContext::new(&query_result, root, services.clone()) {
                Ok(ctx) => ctx,
                Err(_) => return ControlFlow::Continue(()),
            };

            for result in R::run(&ctx) {
                let signal =
                    RuleSignal::<R>::new(file_id, root, &query_result, result, services.clone());
                callback(&signal)?;
            }

            ControlFlow::Continue(())
        }

        Self {
            name: R::NAME,
            docs: R::DOCS,
            run: run::<R, B>,
        }
    }
}
