use crate::{react::hooks::react_hook_configuration, semantic_services::Semantic};
use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{
    JsCallExpression, JsFunctionBody, JsFunctionDeclaration, JsSyntaxKind, JsSyntaxNode, TextRange,
};
use rome_rowan::AstNode;

use super::use_exhaustive_dependencies::ReactExtensiveDependenciesOptions;

declare_rule! {
    /// Enforce that all React hooks are being called from the Top Level
    /// component functions.
    ///
    /// To understand why this required see https://reactjs.org/docs/hooks-rules.html#only-call-hooks-at-the-top-level
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function Component1({ a }) {
    ///     if (a == 1) {
    ///         useEffect();
    ///     }
    /// }
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// function Component1() {
    ///     useEffect();
    /// }
    /// ```
    ///
    pub(crate) UseHookAtTopLevel {
        version: "12.0.0",
        name: "useHookAtTopLevel",
        recommended: false,
    }
}

pub enum Suggestion {
    None {
        hook_name_range: TextRange,
        path: Vec<TextRange>,
    },
}

fn is_top_level(node: &JsSyntaxNode) -> Option<JsFunctionDeclaration> {
    let next = node.ancestors().find(|x| {
        !matches!(
            x.kind(),
            JsSyntaxKind::JS_STATEMENT_LIST
                | JsSyntaxKind::JS_BLOCK_STATEMENT
                | JsSyntaxKind::JS_VARIABLE_STATEMENT
                | JsSyntaxKind::JS_EXPRESSION_STATEMENT
                | JsSyntaxKind::JS_CALL_EXPRESSION
                | JsSyntaxKind::JS_INITIALIZER_CLAUSE
                | JsSyntaxKind::JS_VARIABLE_DECLARATOR
                | JsSyntaxKind::JS_VARIABLE_DECLARATOR_LIST
                | JsSyntaxKind::JS_VARIABLE_DECLARATION
        )
    });

    next.and_then(JsFunctionBody::cast)
        .and_then(|body| body.parent::<JsFunctionDeclaration>())
}

pub struct CallPath {
    call: JsCallExpression,
    path: Vec<TextRange>,
}

impl Rule for UseHookAtTopLevel {
    type Query = Semantic<JsCallExpression>;
    type State = Suggestion;
    type Signals = Option<Self::State>;
    type Options = ReactExtensiveDependenciesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let options = ctx.options();

        let call = ctx.query();
        let hook_name_range = call.callee().ok()?.syntax().text_trimmed_range();
        if react_hook_configuration(call, &options.hooks_config).is_some() {
            let model = ctx.model();

            let root = CallPath {
                call: call.clone(),
                path: vec![],
            };
            let mut calls = vec![root];
            while let Some(CallPath { call, path }) = calls.pop() {
                let range = call.syntax().text_range();
                let mut path = path.clone();
                path.push(range);

                if let Some(f) = is_top_level(call.syntax()) {
                    for call in model.all_calls(&f) {
                        let node = call.tree();
                        let path = path.clone();
                        calls.push(CallPath { call: node, path });
                    }
                } else {
                    return Some(Suggestion::None {
                        hook_name_range,
                        path,
                    });
                }
            }
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, suggestion: &Self::State) -> Option<RuleDiagnostic> {
        match suggestion {
            Suggestion::None {
                hook_name_range,
                path,
            } => {
                let call_deep = path.len() - 1;

                let mut diag = if call_deep == 0 {
                    RuleDiagnostic::new(
                        rule_category!(),
                        hook_name_range,
                        markup! {
                            "This hook is not necessarily being called from the Top Level of the component function."
                        },
                    )
                } else {
                    RuleDiagnostic::new(
                        rule_category!(),
                        hook_name_range,
                        markup! {
                            "This hook is being called indirectly and it is not necessarily being called from the Top Level of the component function."
                        },
                    )
                };

                for (i, range) in path.iter().skip(1).enumerate() {
                    let msg = if i == 0 {
                        markup! {
                            "This is the call path until the hook."
                        }
                    } else {
                        markup! {}
                    };

                    diag = diag.detail(range, msg);
                }

                let diag = diag.note(
                    markup! {
                        "For React to preserve state between calls, hooks needs to be called always in the same order."
                    },
                ).note(
                    markup! {
                        "See https://reactjs.org/docs/hooks-rules.html#only-call-hooks-at-the-top-level"
                    },
                );
                Some(diag)
            }
        }
    }
}
