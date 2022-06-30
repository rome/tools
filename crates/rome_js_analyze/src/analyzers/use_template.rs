use rome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleCategory, RuleDiagnostic,
};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyExpression, JsAnyStatement, JsBinaryExpression, JsBinaryOperator, JsForStatement,
    JsForStatementFields, T,
};
use rome_rowan::{AstNode, AstNodeExt, declare_node_union};

use crate::{utils::interpret_escaped_string, JsRuleAction};

declare_rule! {
    /// Enforce the use of `while` loops instead of `for` loops when the
    /// initializer and update expressions are not needed
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// for (; x.running;) {
    ///     x.step();
    /// }
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
        // todo!()
        let Ast(node) = ctx.query();

        // SAFETY: These tokens have been checked for errors in `run` already

        Some(RuleDiagnostic::warning(
            node.range(),
            markup! {
                "Template literals are preferred over string concatenation."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let Ast(node) = ctx.query();
        let root = ctx.root();

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Use a while loop" }.to_owned(),
            root,
        })
    }
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
            // println!("{:?}", string_literal.text().chars().collect::<Vec<_>>());
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
            // I don't know which one would have more overhead, loop string twice or allocation a new string.
            // concat string usually would be short, so I think it's would have lower overhead than allocate a string.
            // println!("string_lit: {}", string_literal.text());
            // println!(
            //     "{:?}, {:?} ",
            //     interpret_escaped_string(&string_literal.inner_string_text().to_string()).and_then(|item| interpret_escaped_string(&item)),
            //     string_literal.text().chars().collect::<Vec<_>>()
            // );
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

// function flattenExpressionsToTemplateParts(
// 	expressions: AnyJSExpression[],
// ): TemplatePart[] {
// 	let parts: TemplatePart[] = [];
// 	let queue: TemplatePart[] = [...expressions];

// 	while (true) {
// 		let node = queue.shift();
// 		if (!node) {
// 			break;
// 		}

// 		if (node.type === "JSTemplateLiteral") {
// 			queue = [...zipTemplateLiteralParts(node), ...queue];
// 		} else {
// 			parts.push(node);
// 		}
// 	}

// 	return parts;
// }


fn flatten_expressions_to_template_parts(exprs: Vec<JsAnyExpression>) -> Vec<>  {
    let mut parts = vec![];
    let mut queue = vec![];
    queue.extend(exprs);

    while let Some(node) = queue.pop() {
        if let JsAnyExpression::JsTemplate(template) = node {
            queue.extend(template.expressions().iter().cloned());
        } else {
            parts.push(node);
        }
    }

    parts
}

declare_node_union! {

    /// Matches an if statement or a conditional expression
    pub(crate) TemplatePart = JsAnyExpression | JsTemplate
}