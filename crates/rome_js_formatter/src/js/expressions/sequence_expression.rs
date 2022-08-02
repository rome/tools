use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::JsSyntaxKind::{JS_PARENTHESIZED_EXPRESSION, JS_SEQUENCE_EXPRESSION};
use rome_js_syntax::{JsSequenceExpression, JsSequenceExpressionFields, JsSyntaxKind};
use rome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub struct FormatJsSequenceExpression;

impl FormatNodeRule<JsSequenceExpression> for FormatJsSequenceExpression {
    fn fmt_fields(&self, node: &JsSequenceExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsSequenceExpressionFields {
            left,
            comma_token,
            right,
        } = node.as_fields();

        let mut is_nested = false;
        let mut first_non_sequence_or_paren_parent = None;

        for parent in node.syntax().ancestors().skip(1) {
            if parent.kind() == JS_SEQUENCE_EXPRESSION {
                is_nested = true;
            } else if parent.kind() != JS_PARENTHESIZED_EXPRESSION {
                first_non_sequence_or_paren_parent = Some(parent);
                break;
            }
        }

        let format_inner = format_with(|f| {
            if let Some(parent) = &first_non_sequence_or_paren_parent {
                if matches!(
                    parent.kind(),
                    JsSyntaxKind::JS_EXPRESSION_STATEMENT | JsSyntaxKind::JS_FOR_STATEMENT
                ) {
                    return write!(
                        f,
                        [
                            left.format(),
                            comma_token.format(),
                            line_suffix_boundary(),
                            soft_line_indent_or_space(&right.format())
                        ]
                    );
                }
            }

            write!(
                f,
                [
                    left.format(),
                    comma_token.format(),
                    line_suffix_boundary(),
                    soft_line_break_or_space(),
                    right.format()
                ]
            )
        });

        if is_nested {
            write!(f, [format_inner])
        } else {
            write!(f, [group(&format_inner)])
        }
    }
}
