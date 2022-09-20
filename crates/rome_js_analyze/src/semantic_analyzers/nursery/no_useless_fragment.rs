use crate::semantic_services::Semantic;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Rule, RuleCategory, RuleDiagnostic};
use rome_console::markup;
use rome_js_semantic::SemanticModel;
use rome_js_syntax::JsSyntaxKind::JS_IMPORT;
use rome_js_syntax::{
    JsIdentifierBinding, JsImport, JsSyntaxKind, JsxAnyChild, JsxAnyElementName, JsxChildList,
    JsxElement, JsxFragment, JsxMemberName, JsxReferenceIdentifier, JsxTagExpression,
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
    /// <></>
    /// ```
    pub(crate) NoUselessFragment {
        version: "0.10.0",
        name: "noUselessFragment",
        recommended: false,
    }
}

#[derive(Debug)]
pub(crate) enum NoUselessFragmentState {
    Empty,
    Attribute(JsxAnyChild),
}

declare_node_union! {
    pub(crate) NoUselessFragmentQuery = JsxFragment | JsxElement
}

impl Rule for NoUselessFragment {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = Semantic<NoUselessFragmentQuery>;
    type State = NoUselessFragmentState;
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
        match node {
            NoUselessFragmentQuery::JsxFragment(fragment) => {
                let parent = node.syntax().parent().and_then(|parent| {
                    if let Some(parent) = JsxTagExpression::cast_ref(&parent) {
                        parent.syntax().parent()
                    } else {
                        Some(parent)
                    }
                });

                let parent_kind = parent.map(|p| p.kind());
                let matches_allowed_parents = matches!(
                    parent_kind,
                    Some(
                        JsSyntaxKind::JS_RETURN_STATEMENT
                            | JsSyntaxKind::JS_INITIALIZER_CLAUSE
                            | JsSyntaxKind::JS_CONDITIONAL_EXPRESSION
                            | JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION
                            | JsSyntaxKind::JS_FUNCTION_EXPRESSION
                            | JsSyntaxKind::JS_FUNCTION_DECLARATION
                    )
                );

                let child_list = fragment.children();
                if child_list.len() <= 1 && !matches_allowed_parents {
                    return if child_list.is_empty() {
                        Some(NoUselessFragmentState::Empty)
                    } else {
                        // SAFETY: it has at least one item
                        let attribute = child_list.first().unwrap();
                        Some(NoUselessFragmentState::Attribute(attribute))
                    };
                }

                None
            }
            NoUselessFragmentQuery::JsxElement(element) => {
                let opening_element = element.opening_element().ok()?;
                let name = opening_element.name().ok()?;

                return match name {
                    JsxAnyElementName::JsxMemberName(member_name) => {
                        if jsx_member_name_is_react_fragment(&member_name, model)? {
                            Some(NoUselessFragmentState::Empty)
                        } else {
                            None
                        }
                    }
                    JsxAnyElementName::JsxReferenceIdentifier(identifier) => {
                        if jsx_reference_identifier_is_fragment(&identifier, model)? {
                            Some(NoUselessFragmentState::Empty)
                        } else {
                            None
                        }
                    }
                    JsxAnyElementName::JsxName(_) | JsxAnyElementName::JsxNamespaceName(_) => None,
                };
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

fn jsx_member_name_is_react_fragment(
    member_name: &JsxMemberName,
    model: &SemanticModel,
) -> Option<bool> {
    let object = member_name.object().ok()?;
    let member = member_name.member().ok()?;
    let object = object.as_jsx_reference_identifier()?;
    let maybe_react_fragment = object.value_token().ok()?.text_trimmed() == "React"
        && member.value_token().ok()?.text_trimmed() == "Fragment";
    if maybe_react_fragment {
        let reference = model.declaration(object);
        if let Some(reference) = reference {
            for ancestor in reference.syntax().ancestors() {
                if let Some(js_import) = JsImport::cast(ancestor) {
                    let source_is_react = js_import.source_is("react").ok()?;
                    let reference = JsIdentifierBinding::cast_ref(reference.syntax())?;
                    return Some(
                        reference.name_token().ok()?.text_trimmed()
                            == object.value_token().ok()?.text_trimmed()
                            && source_is_react,
                    );
                }
            }
            return None;
        }
    }

    Some(maybe_react_fragment)
}

fn jsx_reference_identifier_is_fragment(
    name: &JsxReferenceIdentifier,
    model: &SemanticModel,
) -> Option<bool> {
    let value_token = name.value_token().ok()?;
    let maybe_react_fragment = value_token.text_trimmed() == "Fragment";
    if maybe_react_fragment {
        let reference = model.declaration(name);
        if let Some(reference) = reference {
            for ancestor in reference.syntax().ancestors() {
                if let Some(js_import) = JsImport::cast(ancestor) {
                    let source_is_react = js_import.source_is("react").ok()?;
                    let reference = JsIdentifierBinding::cast_ref(reference.syntax())?;
                    return Some(
                        reference.name_token().ok()?.text_trimmed() == value_token.text_trimmed()
                            && source_is_react,
                    );
                }
            }
            return None;
        }
    }

    Some(maybe_react_fragment)
}
