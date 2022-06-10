use rome_console::MarkupBuf;
use rome_diagnostics::{file::FileId, Applicability, Severity};
use rome_js_syntax::JsLanguage;
use rome_js_syntax::TextRange;
use rome_rowan::{AstNode, Language, SyntaxNode};

use crate::{
    analyzers::*,
    assists::*,
    categories::{ActionCategory, RuleCategory},
    signals::{AnalyzerSignal, RuleSignal},
    AnalysisFilter, ControlFlow,
};

/// The rule registry holds type-erased instances of all active analysis rules
pub(crate) struct RuleRegistry<L: Language> {
    rules: Vec<RegistryRule<L>>,
}

/// Utility macro for implementing the `with_filter` method of [RuleRegistry]
macro_rules! impl_registry_builders {
    ( $( $rule:ident, )* ) => {
        impl RuleRegistry<JsLanguage> {
            pub(crate) fn with_filter(filter: &AnalysisFilter) -> Self {
                let mut rules: Vec<RegistryRule<JsLanguage>> = Vec::new();

                $( if filter.categories.contains($rule::CATEGORY.into()) && filter.rules.map_or(true, |rules| rules.contains(&$rule::NAME)) {
                    rules.push(run::<$rule>);
                } )*

                Self { rules }
            }
        }
    };
}

impl_registry_builders!(
    // Analyzers
    NoCompareNegZero,
    NoDelete,
    NoDoubleEquals,
    NoNegationElse,
    NoUnusedTemplateLiteral,
    UseSingleCaseStatement,
    UseSingleVarDeclarator,
    UseValidTypeof,
    UseWhile,
    // Assists
    FlipBinExp,
);

pub(crate) type RuleLanguage<R> = NodeLanguage<<R as Rule>::Query>;
pub(crate) type NodeLanguage<N> = <N as AstNode>::Language;

pub(crate) type RuleRoot<R> = LanguageRoot<RuleLanguage<R>>;
pub(crate) type LanguageRoot<L> = <L as Language>::Root;

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
            if let Some(event) = (rule)(file_id, root, &node) {
                if let ControlFlow::Break(b) = callback(&*event) {
                    return ControlFlow::Break(b);
                }
            }
        }

        ControlFlow::Continue(())
    }
}

/// Representation of a single rule in the registry as a generic function pointer
type RegistryRule<L> = for<'a> fn(
    FileId,
    &'a LanguageRoot<L>,
    &'a SyntaxNode<L>,
) -> Option<Box<dyn AnalyzerSignal<L> + 'a>>;

/// Generic implementation of RegistryRule for any rule type R
fn run<'a, R: Rule + 'static>(
    file_id: FileId,
    root: &'a RuleRoot<R>,
    node: &'a SyntaxNode<<R::Query as AstNode>::Language>,
) -> Option<Box<dyn AnalyzerSignal<RuleLanguage<R>> + 'a>> {
    if !<R::Query>::can_cast(node.kind()) {
        return None;
    }

    let node = <R::Query>::cast(node.clone())?;
    let result = R::run(&node)?;
    Some(RuleSignal::<R>::new_boxed(file_id, root, node, result))
}

/// Trait implemented by all analysis rules: declares interest to a certain AstNode type,
/// and a callback function to be executed on all nodes matching the query to possibly
/// raise an analysis event
pub(crate) trait Rule {
    /// The name of this rule, displayed in the diagnostics it emits
    const NAME: &'static str;
    /// The category this rule belong to, this is used for broadly filtering
    /// rules when running the analyzer
    const CATEGORY: RuleCategory;

    /// The type of AstNode this rule is interested in
    type Query: AstNode;
    /// A generic type that will be kept in memory between a call to `run` and
    /// subsequent executions of `diagnostic` or `action`, allows the rule to
    /// hold some temporary state between the moment a signal is raised and
    /// when a diagnostic or action needs to be built
    type State;

    /// This function is called once for each node matching `Query` in the tree
    /// being analyzed. If it returns `Some` the state object will be wrapped
    /// in a generic `AnalyzerSignal`, and the consumer of the analyzer may call
    /// `diagnostic` or `action` on it
    fn run(node: &Self::Query) -> Option<Self::State>;

    /// Called by the consumer of the analyzer to try to generate a diagnostic
    /// from a signal raised by `run`
    ///
    /// The default implementation returns None
    fn diagnostic(_node: &Self::Query, _state: &Self::State) -> Option<RuleDiagnostic> {
        None
    }

    /// Called by the consumer of the analyzer to try to generate a code action
    /// from a signal raised by `run`
    ///
    /// The default implementation returns None
    fn action(
        _root: RuleRoot<Self>,
        _node: &Self::Query,
        _state: &Self::State,
    ) -> Option<RuleAction<RuleLanguage<Self>>> {
        None
    }
}

/// Diagnostic object returned by a single analysis rule
pub struct RuleDiagnostic {
    pub severity: Severity,
    pub range: TextRange,
    pub message: MarkupBuf,
}

/// Code Action object returned by a single analysis rule
pub struct RuleAction<L: Language> {
    pub category: ActionCategory,
    pub applicability: Applicability,
    pub message: MarkupBuf,
    pub root: LanguageRoot<L>,
}

pub type JsRuleAction = RuleAction<JsLanguage>;
