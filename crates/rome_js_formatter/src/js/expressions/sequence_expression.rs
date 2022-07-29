use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::JsSyntaxKind::JS_SEQUENCE_EXPRESSION;
use rome_js_syntax::{JsSequenceExpression, JsSequenceExpressionFields, JsSyntaxKind};
use rome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub struct FormatJsSequenceExpression;

impl FormatNodeRule<JsSequenceExpression> for FormatJsSequenceExpression {
    fn fmt_fields(&self, node: &JsSequenceExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let first_non_sequence_parent = node
            .syntax()
            .ancestors()
            .find(|p| p.kind() != JS_SEQUENCE_EXPRESSION);

        let is_nested = first_non_sequence_parent != node.syntax().parent();

        let has_already_indentation = first_non_sequence_parent.map_or(false, |parent| {
            match parent.kind() {
                // Return statement already does the indentation for us
                // Arrow function body can't have a sequence expression unless it's parenthesized, otherwise
                // would be a syntax error
                JsSyntaxKind::JS_RETURN_STATEMENT => true,
                JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION => {
                    // In case we are inside a sequence expression, we have to go up a level and see the great parent.
                    // Arrow function body and return statements applying indentation for us, so we signal the
                    // sequence expression to not add other indentation levels
                    let great_parent = parent.parent().map(|gp| gp.kind());

                    matches!(
                        great_parent,
                        Some(
                            JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION
                                | JsSyntaxKind::JS_RETURN_STATEMENT
                                | JsSyntaxKind::JS_PROPERTY_OBJECT_MEMBER
                        )
                    )
                }
                _ => false,
            }
        });

        let JsSequenceExpressionFields {
            left,
            comma_token,
            right,
        } = node.as_fields();

        let format_content = format_with(|f| {
            write!(f, [left.format(), comma_token.format()])?;

            let format_right =
                format_with(|f| write!(f, [soft_line_break_or_space(), right.format()]));

            if has_already_indentation {
                write!(f, [format_right])
            } else {
                write!(f, [indent(&format_right)])
            }
        });

        if is_nested {
            write!(f, [format_content])
        } else {
            write!(f, [group(&format_content)])
        }
    }
}
