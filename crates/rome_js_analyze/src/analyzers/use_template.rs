use rome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleCategory, RuleDiagnostic,
};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::JsAnyTemplateElement::{self, JsTemplateElement};
use rome_js_syntax::{
    JsAnyExpression, JsAnyLiteralExpression, JsBinaryExpression, JsBinaryOperator, JsSyntaxKind,
    JsSyntaxToken, JsTemplate, JsTemplateElementList, T,
};
use rome_rowan::{AstNode, AstNodeExt, AstNodeList, SyntaxToken};

use crate::{utils::interpret_escaped_string, JsRuleAction};

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
    /// ```js,expect_diagnostic
    /// console.log(1 * 2 + "foo");
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
    pub(crate) UseTemplate = "useTemplate"
}

impl Rule for UseTemplate {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = Ast<JsBinaryExpression>;
    type State = Vec<JsAnyExpression>;
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let Ast(binary_expr) = ctx.query();
        is_un_necessary_string_concat_expression(binary_expr).and_then(|result| {
            if result {
                let collections = collect_binary_add_expression(binary_expr)?;
                if collections.iter().any(|expr| {
                    !matches!(
                        expr,
                        JsAnyExpression::JsAnyLiteralExpression(
                            rome_js_syntax::JsAnyLiteralExpression::JsStringLiteralExpression(_)
                        )
                    )
                }) {
                    return Some(collections);
                }
            }
            None
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let Ast(node) = ctx.query();

        Some(RuleDiagnostic::warning(
            node.range(),
            markup! {
                ""<Emphasis>"Template"</Emphasis>" literals are preferred over "<Emphasis>"string concatenation."</Emphasis>""
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        // return None;
        let Ast(node) = ctx.query();
        let root = ctx.root();

        let template = convert_expressions_to_js_template(state)?;
        let next_root = root.replace_node(
            JsAnyExpression::JsBinaryExpression(node.clone()),
            JsAnyExpression::JsTemplate(template),
        )?;
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Use a "<Emphasis>"TemplateLiteral"</Emphasis>"." }.to_owned(),
            root: next_root,
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
                        string_without_quotes,
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
                // drop the trivia of original expression make the generated `JsTemplate` a little nicer, if we don't do this
                // the `1 * (2 + "foo") + "bar"` will become
                // ```js
                // `${1 * (2 + "foo") }bar`
                // ```
                let expr_next = expr.clone();
                let first_token = expr_next.syntax().first_token()?;
                let expr_next = expr_next.replace_token_discard_trivia(
                    first_token.clone(),
                    first_token.with_leading_trivia(std::iter::empty()),
                )?;
                let last_token = expr_next.syntax().last_token()?;
                let expr_next = expr_next.replace_token_discard_trivia(
                    last_token.clone(),
                    last_token.with_trailing_trivia(std::iter::empty()),
                )?;
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
fn is_un_necessary_string_concat_expression(node: &JsBinaryExpression) -> Option<bool> {
    if node.operator().ok()? != JsBinaryOperator::Plus {
        return None;
    }
    match node.left().ok()? {
        rome_js_syntax::JsAnyExpression::JsBinaryExpression(binary) => {
            if is_un_necessary_string_concat_expression(&binary) == Some(true) {
                return Some(true);
            }
        }
        rome_js_syntax::JsAnyExpression::JsTemplate(_) => return Some(true),
        rome_js_syntax::JsAnyExpression::JsAnyLiteralExpression(
            rome_js_syntax::JsAnyLiteralExpression::JsStringLiteralExpression(string_literal),
        ) => {
            // I don't know which one would have more overhead, loop string twice or allocation a new string.
            // concat string usually would be short, so I think it's would have lower overhead than allocate a string.
            if interpret_escaped_string(&string_literal.text())
                .ok()?
                .find(|ch| matches!(ch, '\n' | '`'))
                .is_none()
            {
                return Some(true);
            }
        }
        _ => {}
    }
    match node.right().ok()? {
        rome_js_syntax::JsAnyExpression::JsBinaryExpression(binary) => {
            if is_un_necessary_string_concat_expression(&binary) == Some(true) {
                return Some(true);
            }
        }
        rome_js_syntax::JsAnyExpression::JsTemplate(_) => return Some(true),
        rome_js_syntax::JsAnyExpression::JsAnyLiteralExpression(
            rome_js_syntax::JsAnyLiteralExpression::JsStringLiteralExpression(string_literal),
        ) => {
            if interpret_escaped_string(&string_literal.text())
                .ok()?
                .find(|ch| matches!(ch, '\n' | '`'))
                .is_none()
            {
                return Some(true);
            }
        }
        _ => {}
    }
    None
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
            result.extend(collect_binary_add_expression(&left)?);
        }
        left => {
            result.push(left);
        }
    };
    match node.right().ok()? {
        JsAnyExpression::JsBinaryExpression(right)
            if matches!(right.operator().ok()?, JsBinaryOperator::Plus) =>
        {
            result.extend(collect_binary_add_expression(&right)?);
        }
        right => {
            result.push(right);
        }
    };
    Some(result)
}
