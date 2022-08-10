use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use rome_formatter::{format_args, write};
use rome_js_syntax::JsSyntaxKind::{JS_PARENTHESIZED_EXPRESSION, JS_SEQUENCE_EXPRESSION};
use rome_js_syntax::{
    JsSequenceExpression, JsSequenceExpressionFields, JsSyntaxKind, JsSyntaxNode,
};
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

        // Skip 1 because ancestor starts with the current node but we're interested in the parent
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
                            indent(&format_args![soft_line_break_or_space(), right.format()])
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

    fn needs_parentheses(&self, item: &JsSequenceExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for JsSequenceExpression {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        match parent.kind() {
            JsSyntaxKind::JS_RETURN_STATEMENT => false,
            // There's a precedence for writing `x++, y++`
            JsSyntaxKind::JS_FOR_STATEMENT => false,
            JsSyntaxKind::JS_EXPRESSION_STATEMENT => false,
            JsSyntaxKind::JS_SEQUENCE_EXPRESSION => false,
            // Be on the safer side
            _ => true,
        }
        // case "SequenceExpression":
        //       switch (parent.type) {
        //
        //         case "ExpressionStatement":
        //           return name !== "expression";
        //
        //         case "ArrowFunctionExpression":
        //           // We do need parentheses, but SequenceExpressions are handled
        //           // specially when printing bodies of arrow functions.
        //           return name !== "body";
        //
        //         default:
        //           // Otherwise err on the side of overparenthesization, adding
        //           // explicit exceptions above if this proves overzealous.
        //           return true;
        //       }
    }
}
