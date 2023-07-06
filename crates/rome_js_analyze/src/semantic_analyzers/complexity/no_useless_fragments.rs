use crate::react::{jsx_member_name_is_react_fragment, jsx_reference_identifier_is_fragment};
use crate::semantic_services::Semantic;
use crate::JsRuleAction;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make::{ident, js_expression_statement, jsx_string, jsx_tag_expression};
use rome_js_syntax::{
    AnyJsxChild, AnyJsxElementName, AnyJsxTag, JsLanguage, JsParenthesizedExpression, JsSyntaxKind,
    JsxChildList, JsxElement, JsxFragment, JsxTagExpression,
};
use rome_rowan::{declare_node_union, AstNode, AstNodeList, BatchMutation, BatchMutationExt};

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
    /// <>
    ///     <>foo</>
    ///     <SomeComponent />
    /// </>
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
    Child(AnyJsxChild),
}

declare_node_union! {
    pub(crate) NoUselessFragmentsQuery = JsxFragment | JsxElement
}

impl NoUselessFragmentsQuery {
    fn replace_node(&self, mutation: &mut BatchMutation<JsLanguage>, new_node: AnyJsxChild) {
        match self {
            NoUselessFragmentsQuery::JsxFragment(fragment) => {
                let old_node = AnyJsxChild::JsxFragment(fragment.clone());
                mutation.replace_node(old_node, new_node);
            }
            NoUselessFragmentsQuery::JsxElement(element) => {
                let old_node = AnyJsxChild::JsxElement(element.clone());
                mutation.replace_node(old_node, new_node);
            }
        }
    }

    fn remove_node_from_list(&self, mutation: &mut BatchMutation<JsLanguage>) {
        match self {
            NoUselessFragmentsQuery::JsxFragment(fragment) => {
                let old_node = AnyJsxChild::JsxFragment(fragment.clone());
                mutation.remove_node(old_node);
            }
            NoUselessFragmentsQuery::JsxElement(element) => {
                let old_node = AnyJsxChild::JsxElement(element.clone());
                mutation.remove_node(old_node);
            }
        }
    }

    fn children(&self) -> JsxChildList {
        match self {
            NoUselessFragmentsQuery::JsxFragment(element) => element.children(),
            NoUselessFragmentsQuery::JsxElement(element) => element.children(),
        }
    }
}

impl Rule for NoUselessFragments {
    type Query = Semantic<NoUselessFragmentsQuery>;
    type State = NoUselessFragmentsState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
        match node {
            NoUselessFragmentsQuery::JsxFragment(fragment) => {
                let parents_where_fragments_must_be_preserved = node
                    .syntax()
                    .parent()
                    .map(|parent| match JsxTagExpression::try_cast(parent) {
                        Ok(parent) => parent
                            .syntax()
                            .parent()
                            .and_then(|parent| {
                                if let Some(parenthesized_expression) =
                                    JsParenthesizedExpression::cast_ref(&parent)
                                {
                                    parenthesized_expression.syntax().parent()
                                } else {
                                    Some(parent)
                                }
                            })
                            .map(|parent| {
                                matches!(
                                    parent.kind(),
                                    JsSyntaxKind::JS_RETURN_STATEMENT
                                        | JsSyntaxKind::JS_INITIALIZER_CLAUSE
                                        | JsSyntaxKind::JS_CONDITIONAL_EXPRESSION
                                        | JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION
                                        | JsSyntaxKind::JS_FUNCTION_EXPRESSION
                                        | JsSyntaxKind::JS_FUNCTION_DECLARATION
                                        | JsSyntaxKind::JS_PROPERTY_OBJECT_MEMBER
                                )
                            })
                            .unwrap_or(false),
                        Err(_) => false,
                    })
                    .unwrap_or(false);

                let child_list = fragment.children();

                if !parents_where_fragments_must_be_preserved {
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

                let is_valid_react_fragment = match name {
                    AnyJsxElementName::JsxMemberName(member_name) => {
                        jsx_member_name_is_react_fragment(&member_name, model)?
                    }
                    AnyJsxElementName::JsxReferenceIdentifier(identifier) => {
                        jsx_reference_identifier_is_fragment(&identifier, model)?
                    }
                    AnyJsxElementName::JsxName(_) | AnyJsxElementName::JsxNamespaceName(_) => false,
                };

                if is_valid_react_fragment {
                    let child_list = element.children();
                    // The `Fragment` component supports only the "key" prop and react emits a warning for not supported props.
                    // We assume that the user knows - and fixed - that and only care about the prop that is actually supported.
                    let attribute_key =
                        opening_element
                            .attributes()
                            .into_iter()
                            .find_map(|attribute| {
                                let attribute = attribute.as_jsx_attribute()?;
                                let attribute_name = attribute.name().ok()?;
                                let attribute_name = attribute_name.as_jsx_name()?;

                                if attribute_name.value_token().ok()?.text_trimmed() == "key" {
                                    Some(())
                                } else {
                                    None
                                }
                            });
                    if attribute_key.is_none() {
                        return match child_list.first() {
                            Some(first) if child_list.len() == 1 => {
                                Some(NoUselessFragmentsState::Child(first))
                            }
                            None => Some(NoUselessFragmentsState::Empty),
                            _ => None,
                        };
                    }
                }

                None
            }
        }
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        let is_in_list = node
            .syntax()
            .parent()
            .map_or(false, |parent| JsxChildList::can_cast(parent.kind()));
        if is_in_list {
            let new_child = match state {
                NoUselessFragmentsState::Empty => None,
                NoUselessFragmentsState::Child(child) => Some(child.clone()),
            };

            if let Some(new_child) = new_child {
                node.replace_node(&mut mutation, new_child);
            } else {
                node.remove_node_from_list(&mut mutation);
            }
        } else if let Some(parent) = node.parent::<JsxTagExpression>() {
            let parent = parent.syntax().parent()?;
            let child = node.children().first();
            if let Some(child) = child {
                let new_node = match child {
                    AnyJsxChild::JsxElement(node) => Some(
                        jsx_tag_expression(AnyJsxTag::JsxElement(node))
                            .syntax()
                            .clone(),
                    ),
                    AnyJsxChild::JsxFragment(node) => Some(
                        jsx_tag_expression(AnyJsxTag::JsxFragment(node))
                            .syntax()
                            .clone(),
                    ),
                    AnyJsxChild::JsxSelfClosingElement(node) => Some(
                        jsx_tag_expression(AnyJsxTag::JsxSelfClosingElement(node))
                            .syntax()
                            .clone(),
                    ),
                    AnyJsxChild::JsxText(text) => {
                        let new_value = format!("\"{}\"", text.value_token().ok()?);
                        Some(jsx_string(ident(&new_value)).syntax().clone())
                    }
                    AnyJsxChild::JsxExpressionChild(child) => {
                        child.expression().map(|expression| {
                            js_expression_statement(expression).build().syntax().clone()
                        })
                    }

                    // can't apply a code action because it will create invalid syntax
                    // for example `<>{...foo}</>` would become `{...foo}` which would produce
                    // a syntax error
                    AnyJsxChild::JsxSpreadChild(_) => return None,
                };
                if let Some(new_node) = new_node {
                    mutation.replace_element(parent.into(), new_node.into());
                } else {
                    mutation.remove_element(parent.into());
                }
            } else {
                mutation.remove_element(parent.into());
            }
        }

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Remove the Fragment" }.to_owned(),
            mutation,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.syntax().text_trimmed_range(),
            markup! {
                "Avoid using unnecessary "<Emphasis>"Fragment"</Emphasis>"."
            },
        ))
    }
}
