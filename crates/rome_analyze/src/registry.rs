use rome_console::MarkupBuf;
use rome_diagnostics::Severity;
use rome_js_syntax::{JsAnyRoot, JsSyntaxNode, TextRange};
use rome_rowan::{AstNode, SyntaxNode};

use crate::{
    analyzers::*,
    assists::*,
    categories::ActionCategory,
    signals::{AnalyzerSignal, RuleSignal},
};

/// The rule registry holds type-erased instances of all active analysis rules
pub(crate) struct RuleRegistry {
    rules: Vec<RegistryRule>,
}

/// Utility macro for implementing the `default` and `with_rules` methods of [RuleRegistry]
macro_rules! impl_registry_builders {
    ( $( $rule:ident ),* ) => {
        impl Default for RuleRegistry {
            fn default() -> Self {
                Self {
                    rules: vec![
                        $( run::<$rule>, )*
                    ],
                }
            }
        }

        impl RuleRegistry {
            pub(crate) fn with_rules(filter: &[&str]) -> Self {
                let mut rules: Vec<RegistryRule> = Vec::new();

                $( if filter.contains(&$rule::NAME) {
                    rules.push(run::<$rule>);
                } )*

                Self { rules }
            }
        }
    };
}

impl_registry_builders!(
    // Analyzers
    NoDelete,
    NoDoubleEquals,
    UseSingleVarDeclarator,
    UseWhile,
    // Assists
    FlipBinExp
);

impl RuleRegistry {
    // Run all rules known to the registry associated with nodes of type N
    pub(crate) fn analyze(
        &self,
        root: &JsAnyRoot,
        node: JsSyntaxNode,
        callback: &mut impl FnMut(&dyn AnalyzerSignal),
    ) {
        for rule in &self.rules {
            if let Some(event) = (rule)(root, &node) {
                callback(&*event);
            }
        }
    }
}

/// Representation of a single rule in the registry as a generic function pointer
type RegistryRule =
    for<'a> fn(&'a JsAnyRoot, &'a JsSyntaxNode) -> Option<Box<dyn AnalyzerSignal + 'a>>;

/// Generic implementation of RegistryRule for any rule type R
fn run<'a, R: Rule + 'static>(
    root: &'a JsAnyRoot,
    node: &'a SyntaxNode<<R::Query as AstNode>::Language>,
) -> Option<Box<dyn AnalyzerSignal + 'a>> {
    if !<R::Query>::can_cast(node.kind()) {
        return None;
    }

    let node = <R::Query>::cast(node.clone())?;
    let result = R::run(&node)?;
    Some(RuleSignal::<R>::new_boxed(root, node, result))
}

/// Trait implemented by all analysis rules: declares interest to a certain AstNode type,
/// and a callback function to be executed on all nodes matching the query to possibly
/// raise an analysis event
pub(crate) trait Rule {
    /// The name of this rule, displayed in the diagnostics it emits
    const NAME: &'static str;
    /// The set of categories this rule belong to, this will influence how
    /// clients may chose to present diagnostics and code fixes emitted by this
    /// rule to the user
    const ACTION_CATEGORIES: &'static [ActionCategory];

    /// The type of AstNode this rule is interested in
    type Query: AstNode + 'static;
    /// A generic type that will be kept in memory between a call to `run` and
    /// subsequent executions of `diagnostic` or `code_fix`, allows the rule to
    /// hold some temporary state between the moment a signal is raised and
    /// when a diagnostic or code fix needs to be built
    type State: 'static;

    /// This function is called once for each node matching `Query` in the tree
    /// being analyzed. If it returns `Some` the state object will be wrapped
    /// in a generic `AnalyzerSignal`, and the consumer of the analyzer may call
    /// `diagnostic` or `code_fix` on it
    fn run(node: &Self::Query) -> Option<Self::State>;

    /// Called by the consumer of the analyzer to try to generate a diagnostic
    /// from a signal raised by `run`
    ///
    /// The default implementation returns None
    fn diagnostic(_node: &Self::Query, _state: &Self::State) -> Option<RuleDiagnostic> {
        None
    }

    /// Called by the consumer of the analyzer to try to generate a code fix
    /// from a signal raised by `run`
    ///
    /// The default implementation returns None
    fn code_fix(
        _root: JsAnyRoot,
        _node: &Self::Query,
        _state: &Self::State,
    ) -> Option<RuleCodeFix> {
        None
    }
}

/// Diagnostic object returned by a single analysis rule
pub struct RuleDiagnostic {
    pub severity: Severity,
    pub range: TextRange,
    pub message: MarkupBuf,
}

/// Code fix object returned by a single analysis rule
pub struct RuleCodeFix {
    pub root: JsAnyRoot,
}
