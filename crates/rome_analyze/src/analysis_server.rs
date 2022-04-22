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
    const NAME: &'static str;
    const ACTION_CATEGORIES: &'static [ActionCategory];

    type Query: AstNode + 'static;
    type Result: 'static;

    fn run(node: &Self::Query) -> Option<Self::Result>;

    fn diagnostic(_node: &Self::Query, _result: &Self::Result) -> Option<RuleDiagnostic> {
        None
    }

    fn code_fix(
        _root: &JsAnyRoot,
        _node: &Self::Query,
        _result: &Self::Result,
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
