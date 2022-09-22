use crate::react::{jsx_member_name_is_react_fragment, jsx_reference_identifier_is_fragment};
use crate::semantic_services::Semantic;
use crate::utils::batch::JsBatchMutation;
use crate::JsRuleAction;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Rule, RuleCategory, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_factory::make::{jsx_expression_child, jsx_ident, jsx_text};
use rome_js_syntax::{
    JsLanguage, JsSyntaxKind, JsxAnyChild, JsxAnyElementName, JsxChildList, JsxElement,
    JsxFragment, JsxMemberName, JsxReferenceIdentifier, JsxTagExpression, T,
};
use rome_rowan::{
    declare_node_union, AstNode, AstNodeExt, AstNodeList, BatchMutation, BatchMutationExt,
    SyntaxTriviaPiece,
};

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
    Child(JsxAnyChild),
}

declare_node_union! {
    pub(crate) NoUselessFragmentsQuery = JsxFragment | JsxElement
}

impl NoUselessFragmentsQuery {
    fn replace_node_from_list(
        &self,
        mutation: &mut BatchMutation<JsLanguage>,
        new_node: JsxAnyChild,
    ) {
        match self {
            NoUselessFragmentsQuery::JsxFragment(fragment) => {
                let old_node = JsxAnyChild::JsxFragment(fragment.clone());
                mutation.replace_jsx_child_element(old_node, new_node);
            }
            NoUselessFragmentsQuery::JsxElement(element) => {
                let old_node = JsxAnyChild::JsxElement(element.clone());
                mutation.replace_jsx_child_element(old_node, new_node);
            }
        }
    }

    fn remove_node_from_list(&self, mutation: &mut BatchMutation<JsLanguage>) {
        match self {
            NoUselessFragmentsQuery::JsxFragment(fragment) => {
                let old_node = JsxAnyChild::JsxFragment(fragment.clone());
                mutation.remove_jsx_child_element(old_node);
            }
            NoUselessFragmentsQuery::JsxElement(element) => {
                let old_node = JsxAnyChild::JsxElement(element.clone());
                mutation.remove_jsx_child_element(old_node);
            }
        }
    }

    fn opening_comments(&self) -> Option<Vec<SyntaxTriviaPiece<JsLanguage>>> {
        let mut comments = Vec::new();
        let l_angle = match self {
            NoUselessFragmentsQuery::JsxFragment(fragment) => {
                let opening_fragment = fragment.opening_fragment().ok()?;
                opening_fragment.l_angle_token().ok()?
            }
            NoUselessFragmentsQuery::JsxElement(element) => {
                let opening_element = element.opening_element().ok()?;
                opening_element.l_angle_token().ok()?
            }
        };

        comments.extend(
            l_angle
                .leading_trivia()
                .pieces()
                .filter(|piece| piece.is_comments())
                .chain(
                    l_angle
                        .trailing_trivia()
                        .pieces()
                        .filter(|piece| piece.is_comments()),
                ),
        );

        Some(comments)
    }

    fn closing_comments(&self) -> Option<Vec<SyntaxTriviaPiece<JsLanguage>>> {
        let mut comments = Vec::new();
        let (l_angle, slash) = match self {
            NoUselessFragmentsQuery::JsxFragment(fragment) => {
                let closing_fragment = fragment.closing_fragment().ok()?;
                (
                    closing_fragment.l_angle_token().ok()?,
                    closing_fragment.slash_token().ok()?,
                )
            }
            NoUselessFragmentsQuery::JsxElement(element) => {
                let closing_element = element.closing_element().ok()?;
                (
                    closing_element.l_angle_token().ok()?,
                    closing_element.slash_token().ok()?,
                )
            }
        };

        comments.extend(
            l_angle
                .leading_trivia()
                .pieces()
                .filter(|piece| piece.is_comments())
                .chain(
                    l_angle
                        .trailing_trivia()
                        .pieces()
                        .filter(|piece| piece.is_comments()),
                )
                .chain(
                    slash
                        .leading_trivia()
                        .pieces()
                        .filter(|piece| piece.is_comments()),
                )
                .chain(
                    slash
                        .trailing_trivia()
                        .pieces()
                        .filter(|piece| piece.is_comments()),
                ),
        );

        Some(comments)
    }
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

                let is_valid_react_fragment = match name {
                    JsxAnyElementName::JsxMemberName(member_name) => {
                        jsx_member_name_is_react_fragment(&member_name, model)?
                    }
                    JsxAnyElementName::JsxReferenceIdentifier(identifier) => {
                        jsx_reference_identifier_is_fragment(&identifier, model)?
                    }
                    JsxAnyElementName::JsxName(_) | JsxAnyElementName::JsxNamespaceName(_) => false,
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

        let opening_leading_comments = node.opening_comments()?;
        let closing_leading_comments = node.closing_comments()?;

        let is_in_list = node
            .syntax()
            .parent()
            .map_or(false, |parent| JsxChildList::can_cast(parent.kind()));
        if is_in_list {
            let new_child = match state {
                NoUselessFragmentsState::Empty => {
                    if !opening_leading_comments.is_empty() || !closing_leading_comments.is_empty()
                    {
                        let open_curly = make::token(T!['{'])
                            .with_trailing_trivia_pieces(opening_leading_comments);
                        let close_curly = make::token(T!['}'])
                            .with_leading_trivia_pieces(closing_leading_comments);
                        Some(JsxAnyChild::JsxExpressionChild(
                            jsx_expression_child(open_curly, close_curly).build(),
                        ))
                    } else {
                        None
                    }
                }
                NoUselessFragmentsState::Child(child) => {
                    let child = child.clone();
                    let new_child = match child {
                        JsxAnyChild::JsxText(text) => {
                            // `JsxText` doesn't have any valid tokens to which we can attach comments.
                            // In this case, we create a new `JsxText` with a new content, which will contain
                            // the old comments.
                            // Leading comments are prepended to the existing content, and the trailing comments
                            // are appended to the to the existing content
                            //
                            // Example:
                            // </*comment/*> content </ /*comment*/>
                            //
                            // Will become
                            //
                            // /*comment*/ content /*comment*/
                            let mut new_text = String::new();
                            opening_leading_comments
                                .into_iter()
                                .for_each(|piece| new_text.push_str(piece.text()));
                            new_text.push_str(text.value_token().ok()?.text_trimmed());
                            closing_leading_comments
                                .into_iter()
                                .for_each(|piece| new_text.push_str(piece.text()));

                            let new_jsx_text = jsx_text(jsx_ident(&new_text));
                            JsxAnyChild::JsxText(new_jsx_text)
                        }
                        _ => {
                            let token = child.syntax().first_token()?;
                            let new_token = token
                                .with_leading_trivia_pieces(opening_leading_comments)
                                .with_trailing_trivia_pieces(closing_leading_comments);
                            child.replace_token_discard_trivia(token, new_token)?
                        }
                    };

                    Some(new_child)
                }
            };

            if let Some(new_child) = new_child {
                node.replace_node_from_list(&mut mutation, new_child);
            } else {
                node.remove_node_from_list(&mut mutation);
            }
        }

        Some(JsRuleAction {
            category: ActionCategory::Refactor,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Remove the Fragment" }.to_owned(),
            mutation,
        })
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
