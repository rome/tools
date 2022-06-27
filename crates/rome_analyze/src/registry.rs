use std::{
    hash::{Hash, Hasher},
    ptr,
};

use rome_diagnostics::file::FileId;
use rome_rowan::{AstNode, Language, RawSyntaxKind, SyntaxKind};
use rustc_hash::FxHashSet;

use crate::{
    context::RuleContext,
    query::{QueryKey, QueryMatch, Queryable},
    services::ServiceBag,
    signals::{AnalyzerSignal, RuleSignal},
    ControlFlow, QueryMatcher, Rule,
};

/// Defines all the phases that the [RuleRegistry] supports.
#[repr(usize)]
#[derive(Clone, Copy, Debug)]
pub enum Phases {
    Syntax = 0,
    Semantic = 1,
}

/// Defines which phase a rule will run. This will be defined
/// by the set of services a rule demands.
pub trait Phase {
    fn phase() -> Phases;
}

/// If a rule do not need any service it can run on the syntax phase.
impl Phase for () {
    fn phase() -> Phases {
        Phases::Syntax
    }
}

/// The rule registry holds type-erased instances of all active analysis rules
/// for each phase.
/// What defines a phase is the set of services that a phase offers. Currently
/// we have:
/// - Syntax Phase: No services are offered, thus its rules can be run immediately;
/// - Semantic Phase: Offers the semantic model, thus these rules can only run
/// after the "SemanticModel" is ready, which demands a whole transverse of the parsed tree.
#[derive(Default)]
/// The rule registry holds type-erased instances of all active analysis rules
pub struct RuleRegistry<L: Language> {
    /// Stores metadata information for all the rules in the registry, sorted
    /// alphabetically
    metadata: Vec<RuleMeta>,
    /// Holds a collection of rules for each [SyntaxKind] node type that has
    /// lint rules associated with it for each phase.
    phases: [Vec<SyntaxKindRules<L>>; 2],
    control_flow: Vec<RegistryRule<L>>,

    suppressed_rules: FxHashSet<RuleKey>,
}

impl<L: Language> RuleRegistry<L> {
    /// Add the rule `R` to the list of rules stores in this registry instance
    pub fn push<R>(&mut self)
    where
        R: Rule + 'static,
        R::Query: Queryable<Language = L> + Clone,
    {
        let phase = R::phase() as usize;
        let (meta, rule) = RegistryRule::of::<R>();

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
                    node.rules.push(rule);
                }
            }
            QueryKey::ControlFlowGraph => {
                self.control_flow.push(rule);
            }
        }

        // Find a suitable index to insert the rule in the `metadata` list to
        // keep it sorted alphabetically
        let index = self
            .metadata
            .binary_search_by(|rule| rule.name.cmp(meta.name));

        if let Err(index) = index {
            self.metadata.insert(index, meta);
        }
    }

    /// Returns an iterator over the name and documentation of all active rules
    /// in this instance of the registry
    pub fn metadata(self) -> impl Iterator<Item = (&'static str, &'static str)> {
        self.metadata.into_iter().map(|rule| (rule.name, rule.docs))
    }
}

impl<L: Language> QueryMatcher<L> for RuleRegistry<L> {
    fn insert_suppression(&mut self, name: &str) -> Option<RuleKey> {
        let index = self
            .metadata
            .binary_search_by(|rule| rule.name.cmp(name))
            .ok()?;

        let key = RuleKey(self.metadata[index].name);
        if self.suppressed_rules.insert(key) {
            Some(key)
        } else {
            None
        }
    }

    fn remove_suppressions(&mut self, suppressions: impl IntoIterator<Item = RuleKey>) {
        for key in suppressions {
            self.suppressed_rules.remove(&key);
        }
    }

    fn match_query<'a>(
        &mut self,
        phase: Phases,
        file_id: FileId,
        root: &'a LanguageRoot<L>,
        query: &'a QueryMatch<L>,
        services: &ServiceBag,
        mut emit_signal: impl FnMut(&dyn AnalyzerSignal<L>) -> ControlFlow<()>,
    ) -> ControlFlow<()> {
        let rules = match query {
            QueryMatch::Syntax(node) => {
                // Convert the numerical value of the SyntaxKind to an index in the
                // `syntax` vector
                let RawSyntaxKind(kind) = node.kind().to_raw();
                let kind = usize::from(kind);

                // Lookup the syntax entry corresponding to the SyntaxKind index
                match self.phases[phase as usize].get(kind) {
                    Some(entry) => &entry.rules,
                    None => return ControlFlow::Continue(()),
                }
            }
            QueryMatch::ControlFlowGraph(_) => &self.control_flow,
        };

        // Run all the rules registered to this QueryMatch
        for rule in rules {
            if self.suppressed_rules.contains(&RuleKey(rule.name)) {
                continue;
            }

            (rule.run)(file_id, root, query, services, &mut emit_signal)?;
        }

        ControlFlow::Continue(())
    }
}

/// [SyntaxKindRules] holds a collection of [Rule]s that match a specific [SyntaxKind] value
struct SyntaxKindRules<L: Language> {
    rules: Vec<RegistryRule<L>>,
}

impl<L: Language> SyntaxKindRules<L> {
    fn new() -> Self {
        Self { rules: Vec::new() }
    }
}

pub(crate) type RuleLanguage<R> = QueryLanguage<<R as Rule>::Query>;
pub(crate) type QueryLanguage<N> = <N as Queryable>::Language;
pub(crate) type NodeLanguage<N> = <N as AstNode>::Language;

pub(crate) type RuleRoot<R> = LanguageRoot<RuleLanguage<R>>;
pub type LanguageRoot<L> = <L as Language>::Root;

/// Newtype wrapper around the name of a rule, implementing equality and
/// hashing based on the raw string pointer instead of comparing the string
/// itself
#[derive(Copy, Clone, Debug, Eq)]
pub struct RuleKey(&'static str);

impl PartialEq for RuleKey {
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self.0, other.0)
    }
}

impl Hash for RuleKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        ptr::hash(self.0, state)
    }
}

/// Metadata entry for a rule in the registry
struct RuleMeta {
    name: &'static str,
    docs: &'static str,
}

/// Internal representation of a single rule in the registry
pub struct RegistryRule<L: Language> {
    name: &'static str,
    run: RuleExecutor<L>,
}

/// Executor for rule as a generic function pointer
type RuleExecutor<L> = for<'a> fn(
    FileId,
    &'a LanguageRoot<L>,
    &'a QueryMatch<L>,
    &'a ServiceBag,
    &mut dyn FnMut(&dyn AnalyzerSignal<L>) -> ControlFlow<()>,
) -> ControlFlow<()>;

// These need to be implemented manually because the implementations generated
// by `derive(Copy, Clone)` would require `where: L: Copy + Clone, B: Copy + Clone`
impl<L: Language> Copy for RegistryRule<L> {}
impl<L: Language> Clone for RegistryRule<L> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<L: Language> RegistryRule<L> {
    fn of<R>() -> (RuleMeta, Self)
    where
        R: Rule + 'static,
        R::Query: Queryable<Language = L> + Clone + 'static,
    {
        /// Generic implementation of RuleExecutor for any rule type R
        fn run<'a, R>(
            file_id: FileId,
            root: &'a RuleRoot<R>,
            query: &'a QueryMatch<RuleLanguage<R>>,
            services: &'a ServiceBag,
            emit_signal: &mut dyn FnMut(&dyn AnalyzerSignal<RuleLanguage<R>>) -> ControlFlow<()>,
        ) -> ControlFlow<()>
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
                emit_signal(&signal)?;
            }

            ControlFlow::Continue(())
        }

        (
            RuleMeta {
                name: R::NAME,
                docs: R::DOCS,
            },
            Self {
                name: R::NAME,
                run: run::<R>,
            },
        )
    }
}
