use crate::prelude::*;
use rome_formatter::{FormatRuleWithOptions, GroupId};
use std::convert::Infallible;

use crate::utils::array::write_array_node;

use crate::utils::has_formatter_trivia;
use rome_js_syntax::{JsAnyExpression, JsArrayElementList, JsSyntaxKind};
use rome_rowan::{AstNode, AstSeparatedList};

#[derive(Debug, Clone, Default)]
pub struct FormatJsArrayElementList {
    group_id: Option<GroupId>,
}

impl FormatRuleWithOptions<JsArrayElementList> for FormatJsArrayElementList {
    type Options = Option<GroupId>;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.group_id = options;
        self
    }
}

impl FormatRule<JsArrayElementList> for FormatJsArrayElementList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsArrayElementList, f: &mut JsFormatter) -> FormatResult<()> {
        if !has_formatter_trivia(node.syntax()) && can_print_fill(node) {
            // Using format_separated is valid in this case as can_print_fill does not allow holes
            return f
                .fill()
                .entries(
                    &soft_line_break_or_space(),
                    node.format_separated(JsSyntaxKind::COMMA)
                        .with_group_id(self.group_id),
                )
                .finish();
        }

        write_array_node(node, f)
    }
}

/// Returns true if the provided JsArrayElementList could
/// be "fill-printed" instead of breaking each element on
/// a different line.
///
/// The underlying logic only allows lists of literal expressions
/// with 10 or less characters, potentially wrapped in a "short"
/// unary expression (+, -, ~ or !)
fn can_print_fill(list: &JsArrayElementList) -> bool {
    use rome_js_syntax::JsAnyArrayElement::*;
    use rome_js_syntax::JsAnyExpression::*;
    use rome_js_syntax::JsUnaryOperator::*;

    list.iter().all(|item| match item {
        Ok(JsAnyExpression(JsUnaryExpression(expr))) => {
            match expr.operator() {
                Ok(Plus | Minus | BitwiseNot | LogicalNot) => {}
                _ => return false,
            }

            if let Ok(expr) = expr.argument() {
                is_short_literal(&expr)
            } else {
                false
            }
        }
        Ok(JsAnyExpression(expr)) => is_short_literal(&expr),
        _ => false,
    })
}

/// Returns true if the provided expression is a literal with 10 or less characters
fn is_short_literal(expr: &JsAnyExpression) -> bool {
    match expr {
        JsAnyExpression::JsAnyLiteralExpression(lit) => {
            let token_len = lit
                .syntax()
                .text_trimmed()
                .try_fold_chunks::<_, _, Infallible>(0, |sum, chunk| {
                    // Count actual characters instead of byte length
                    Ok(sum + chunk.chars().count())
                })
                .expect("the above fold operation is infallible");

            token_len <= 10
        }
        _ => false,
    }
}
