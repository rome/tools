use crate::react::{jsx_member_name_is_react_fragment, jsx_reference_identifier_is_fragment};
use crate::semantic_services::Semantic;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Rule, RuleCategory, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{
    JsSyntaxKind, JsxAnyChild, JsxAnyElementName, JsxElement, JsxFragment, JsxTagExpression,
};
use rome_rowan::{declare_node_union, AstNode, AstNodeList};

declare_rule! {
    /// Disallow unnecessary fragments
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <>
    /// foo
    /// </>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <React.Fragment>
    /// foo
    /// </React.Fragment>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <></>
    /// ```
    pub(crate) NoUselessFragments {
        version: "0.10.0",
        name: "noUselessFragments",
        recommended: false,
    }
}

#[derive(Debug)]
pub(crate) enum NoUselessFragmentsState {
    Empty,
    Child(JsxAnyChild),
}

declare_node_union! {
    pub(crate) NoUselessFragmentsQuery = JsxFragment | JsxElement
}

impl Rule for NoUselessFragments {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = Semantic<NoUselessFragmentsQuery>;
    type State = NoUselessFragmentsState;
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
        match node {
            NoUselessFragmentsQuery::JsxFragment(fragment) => {
                let matches_allowed_parents = node
                    .syntax()
                    .parent()
                    .map(|parent| match JsxTagExpression::try_cast(parent) {
                        Ok(parent) => {
                            let parent_kind = parent.syntax().parent().map(|p| p.kind());
                            matches!(
                                parent_kind,
                                Some(
                                    JsSyntaxKind::JS_RETURN_STATEMENT
                                        | JsSyntaxKind::JS_INITIALIZER_CLAUSE
                                        | JsSyntaxKind::JS_CONDITIONAL_EXPRESSION
                                        | JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION
                                        | JsSyntaxKind::JS_FUNCTION_EXPRESSION
                                        | JsSyntaxKind::JS_FUNCTION_DECLARATION
                                )
                            )
                        }
                        Err(_) => false,
                    })
                    .unwrap_or(false);

                let child_list = fragment.children();

                if !matches_allowed_parents {
                    match child_list.first() {
                        Some(first) if child_list.len() == 1 => {
                            Some(NoUselessFragmentsState::Child(first))
                        }
                        None => Some(NoUselessFragmentsState::Empty),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            NoUselessFragmentsQuery::JsxElement(element) => {
                let opening_element = element.opening_element().ok()?;
                let name = opening_element.name().ok()?;

                match name {
                    JsxAnyElementName::JsxMemberName(member_name) => {
                        if jsx_member_name_is_react_fragment(&member_name, model)? {
                            Some(NoUselessFragmentsState::Empty)
                        } else {
                            None
                        }
                    }
                    JsxAnyElementName::JsxReferenceIdentifier(identifier) => {
                        if jsx_reference_identifier_is_fragment(&identifier, model)? {
                            Some(NoUselessFragmentsState::Empty)
                        } else {
                            None
                        }
                    }
                    JsxAnyElementName::JsxName(_) | JsxAnyElementName::JsxNamespaceName(_) => None,
                }
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::new(
            node.syntax().text_trimmed_range(),
            markup! {
                "Avoid using unnecessary "<Emphasis>"Fragment"</Emphasis>"."
            },
        ))
    }
}
