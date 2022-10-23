use rome_analyze::RuleSuppressions;
use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::JsAnyTemplateElement::{self, JsTemplateElement};
use rome_js_syntax::{
    JsAnyExpression, JsAnyLiteralExpression, JsBinaryExpression, JsBinaryOperator, JsLanguage,
    JsSyntaxKind, JsSyntaxToken, JsTemplate, JsTemplateElementList, WalkEvent, T,
};
use rome_rowan::{AstNode, AstNodeExt, AstNodeList, BatchMutationExt, SyntaxToken, TriviaPiece};

use crate::{utils::escape_string, JsRuleAction};

declare_rule! {
    /// Template literals are preferred over string concatenation.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// console.log(foo + "baz");
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// console.log(1 * 2 + "foo");
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// console.log(1 + "foo" + 2 + "bar" + "baz" + 3);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// console.log((1 + "foo") * 2);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// console.log("foo" + 1);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// console.log("foo" + "bar");
    /// console.log(foo() + "\n");
    /// ```
    pub(crate) UseTemplate {
        version: "0.7.0",
        name: "useTemplate",
        recommended: true,
    }
}

impl Rule for UseTemplate {
    type Query = Ast<JsBinaryExpression>;
    type State = Vec<JsAnyExpression>;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let binary_expr = ctx.query();

        let need_process = is_unnecessary_string_concat_expression(binary_expr)?;
        if !need_process {
            return None;
        }

        let collections = collect_binary_add_expression(binary_expr)?;
        collections
            .iter()
            .any(|expr| {
                !matches!(
                    expr,
                    JsAnyExpression::JsAnyLiteralExpression(
                        rome_js_syntax::JsAnyLiteralExpression::JsStringLiteralExpression(_)
                    )
                )
            })
            .then_some(collections)
    }

    fn suppressed_nodes(
        ctx: &RuleContext<Self>,
        _state: &Self::State,
        suppressions: &mut RuleSuppressions<JsLanguage>,
    ) {
        let mut iter = ctx.query().syntax().preorder();
        while let Some(node) = iter.next() {
            if let WalkEvent::Enter(node) = node {
                if node.kind() == JsSyntaxKind::JS_BINARY_EXPRESSION {
                    suppressions.suppress_node(node);
                } else {
                    iter.skip_subtree();
                }
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                ""<Emphasis>"Template"</Emphasis>" literals are preferred over "<Emphasis>"string concatenation."</Emphasis>""
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        let template = convert_expressions_to_js_template(state)?;
        mutation.replace_node(
            JsAnyExpression::JsBinaryExpression(node.clone()),
            JsAnyExpression::JsTemplate(template),
        );

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Use a "<Emphasis>"TemplateLiteral"</Emphasis>"." }.to_owned(),
            mutation,
        })
    }
}

/// Merge `Vec<JsAnyExpression>` into a `JsTemplate`
fn convert_expressions_to_js_template(exprs: &Vec<JsAnyExpression>) -> Option<JsTemplate> {
    let mut reduced_exprs = Vec::with_capacity(exprs.len());
    for expr in exprs.iter() {
        match expr {
            JsAnyExpression::JsAnyLiteralExpression(
                JsAnyLiteralExpression::JsStringLiteralExpression(string),
            ) => {
                let trimmed_string = string.syntax().text_trimmed().to_string();
                let string_without_quotes = &trimmed_string[1..trimmed_string.len() - 1];
                let chunk_element = JsAnyTemplateElement::JsTemplateChunkElement(
                    make::js_template_chunk_element(JsSyntaxToken::new_detached(
                        JsSyntaxKind::TEMPLATE_CHUNK,
                        &escape_dollar_sign(string_without_quotes),
                        [],
                        [],
                    )),
                );
                reduced_exprs.push(chunk_element);
            }
            JsAnyExpression::JsTemplate(template) => {
                reduced_exprs.extend(flatten_template_element_list(template.elements())?);
            }
            _ => {
                // drop the leading and trailing whitespace of original expression make the generated `JsTemplate` a little nicer, if we don't do this
                // the `1 * (2 + "foo") + "bar"` will become
                // ```js
                // `${1 * (2 + "foo") }bar`
                // ```
                let expr_next = expr.clone();
                let first_token = expr_next.syntax().first_token()?;
                // drop the leading whitespace of leading trivia of first token
                // ## Example
                // `1 *    (2 + "foo") /**trailing */             + "bar"`
                //     ^^^^ drop                     ^^^^^^^^^^^^^ drop
                let next_first_token = {
                    let token_kind = first_token.kind();
                    let token_text = first_token.text().trim_start();
                    let leading_trivia = first_token
                        .leading_trivia()
                        .pieces()
                        .skip_while(|item| item.is_newline() || item.is_whitespace())
                        .map(|item| TriviaPiece::new(item.kind(), item.text_len()))
                        .collect::<Vec<_>>();
                    let trailing_trivia = first_token
                        .trailing_trivia()
                        .pieces()
                        .map(|item| TriviaPiece::new(item.kind(), item.text_len()));
                    JsSyntaxToken::new_detached(
                        token_kind,
                        token_text,
                        leading_trivia,
                        trailing_trivia,
                    )
                };
                let expr_next = expr_next
                    .replace_token_discard_trivia(first_token.clone(), next_first_token)?;
                // Drop the trailing whitespace of trailing trivia of last token
                let last_token = expr_next.syntax().last_token()?;
                let next_last_token = {
                    let token_kind = last_token.kind();
                    let mut trailing_trivia = last_token
                        .trailing_trivia()
                        .pieces()
                        .rev()
                        .skip_while(|item| item.is_newline() || item.is_whitespace())
                        .map(|item| TriviaPiece::new(item.kind(), item.text_len()))
                        .collect::<Vec<_>>();
                    trailing_trivia.reverse();
                    let leading_trivia = last_token
                        .leading_trivia()
                        .pieces()
                        .map(|item| TriviaPiece::new(item.kind(), item.text_len()));
                    let token_text = last_token.text().trim_end();
                    JsSyntaxToken::new_detached(
                        token_kind,
                        token_text,
                        leading_trivia,
                        trailing_trivia,
                    )
                };
                let expr_next =
                    expr_next.replace_token_discard_trivia(last_token.clone(), next_last_token)?;
                let template_element =
                    JsAnyTemplateElement::JsTemplateElement(make::js_template_element(
                        SyntaxToken::new_detached(JsSyntaxKind::DOLLAR_CURLY, "${", [], []),
                        expr_next,
                        SyntaxToken::new_detached(JsSyntaxKind::DOLLAR_CURLY, "}", [], []),
                    ));
                reduced_exprs.push(template_element);
            }
        }
    }
    Some(
        make::js_template(
            make::token(T!['`']),
            make::js_template_element_list(reduced_exprs),
            make::token(T!['`']),
        )
        .build(),
    )
}

/// Flatten a [JsTemplateElementList] of [JsTemplate] which could possibly be recursive, into a `Vec<JsAnyTemplateElement>`
/// ## Example
/// flatten
/// ```js
/// `${1 + 2 + `${a}test` }bar`
/// ```
/// into
/// `[1, 2, a, "test", "bar"]`
fn flatten_template_element_list(list: JsTemplateElementList) -> Option<Vec<JsAnyTemplateElement>> {
    let mut ret = Vec::with_capacity(list.len());
    for element in list {
        match element {
            JsAnyTemplateElement::JsTemplateChunkElement(_) => ret.push(element),
            JsTemplateElement(ref ele) => {
                let expr = ele.expression().ok()?;
                match expr {
                    JsAnyExpression::JsTemplate(template) => {
                        ret.extend(flatten_template_element_list(template.elements())?);
                    }
                    _ => {
                        ret.push(element);
                    }
                }
            }
        }
    }
    Some(ret)
}

fn is_unnecessary_string_concat_expression(node: &JsBinaryExpression) -> Option<bool> {
    if node.operator().ok()? != JsBinaryOperator::Plus {
        return None;
    }
    match node.left().ok()? {
        rome_js_syntax::JsAnyExpression::JsBinaryExpression(binary) => {
            if is_unnecessary_string_concat_expression(&binary) == Some(true) {
                return Some(true);
            }
        }
        rome_js_syntax::JsAnyExpression::JsTemplate(_) => return Some(true),
        rome_js_syntax::JsAnyExpression::JsAnyLiteralExpression(
            rome_js_syntax::JsAnyLiteralExpression::JsStringLiteralExpression(string_literal),
        ) => {
            if has_new_line_or_tick(string_literal).is_none() {
                return Some(true);
            }
        }
        _ => (),
    }
    match node.right().ok()? {
        rome_js_syntax::JsAnyExpression::JsBinaryExpression(binary) => {
            if is_unnecessary_string_concat_expression(&binary) == Some(true) {
                return Some(true);
            }
        }
        rome_js_syntax::JsAnyExpression::JsTemplate(_) => return Some(true),
        rome_js_syntax::JsAnyExpression::JsAnyLiteralExpression(
            rome_js_syntax::JsAnyLiteralExpression::JsStringLiteralExpression(string_literal),
        ) => {
            if has_new_line_or_tick(string_literal).is_none() {
                return Some(true);
            }
        }
        _ => (),
    }
    None
}

/// Check if the string literal has new line or tick
fn has_new_line_or_tick(
    string_literal: rome_js_syntax::JsStringLiteralExpression,
) -> Option<usize> {
    escape_string(string_literal.value_token().ok()?.text_trimmed())
        .ok()?
        .find(|ch| matches!(ch, '\n' | '`'))
}

/// Convert [JsBinaryExpression] recursively only if the `operator` is `+` into Vec<[JsAnyExpression]>
/// ## Example
/// - from: `1 + 2 + 3 + (1 * 2)`
/// - to: `[1, 2, 3, (1 * 2)]`
fn collect_binary_add_expression(node: &JsBinaryExpression) -> Option<Vec<JsAnyExpression>> {
    let mut result = vec![];
    match node.left().ok()? {
        JsAnyExpression::JsBinaryExpression(left)
            if matches!(left.operator().ok()?, JsBinaryOperator::Plus) =>
        {
            result.append(&mut collect_binary_add_expression(&left)?);
        }
        left => {
            result.push(left);
        }
    };
    match node.right().ok()? {
        JsAnyExpression::JsBinaryExpression(right)
            if matches!(right.operator().ok()?, JsBinaryOperator::Plus) =>
        {
            result.append(&mut collect_binary_add_expression(&right)?);
        }
        right => {
            result.push(right);
        }
    };
    Some(result)
}

/// Escape dollar sign in raw string.
fn escape_dollar_sign(unescaped_string: &str) -> String {
    let mut chars = unescaped_string.chars().collect::<Vec<_>>();
    for (i, _) in unescaped_string.match_indices("${") {
        if i == 0 {
            chars.insert(0, '\\');
        } else if chars[i - 1] != '\\' {
            chars.insert(i, '\\');
        }
    }
    chars.into_iter().collect::<String>()
}
