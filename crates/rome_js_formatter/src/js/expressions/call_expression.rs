use crate::prelude::*;
use crate::utils::format_call_expression;

use crate::parentheses::{resolve_left_most_expression, NeedsParentheses};
use rome_js_syntax::{JsCallExpression, JsSyntaxKind, JsSyntaxNode};
use rome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub struct FormatJsCallExpression;

impl FormatNodeRule<JsCallExpression> for FormatJsCallExpression {
    fn fmt_fields(&self, node: &JsCallExpression, formatter: &mut JsFormatter) -> FormatResult<()> {
        format_call_expression(node.syntax(), formatter)
    }
}

// impl NeedsParentheses for JsCallExpression {
//     fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
//         match parent.kind() {
//             JsSyntaxKind::JS_EXPRESSION_STATEMENT | JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION => {
//                 self.callee()
//                     .map_or(false, |callee| starts_with_no_lookahead_token(&callee))
//             }
//             _ => {}
//         }
//
//         //  case "CallExpression":
//         //     case "MemberExpression":
//         //     case "TaggedTemplateExpression":
//         //     case "TSNonNullExpression":
//         //       if (
//         //         name === "callee" &&
//         //         (parent.type === "BindExpression" || parent.type === "NewExpression")
//         //       ) {
//         //         let object = node;
//         //         while (object) {
//         //           switch (object.type) {
//         //             case "CallExpression":
//         //             case "OptionalCallExpression":
//         //               return true;
//         //             case "MemberExpression":
//         //             case "OptionalMemberExpression":
//         //             case "BindExpression":
//         //               object = object.object;
//         //               break;
//         //             // tagged templates are basically member expressions from a grammar perspective
//         //             // see https://tc39.github.io/ecma262/#prod-MemberExpression
//         //             case "TaggedTemplateExpression":
//         //               object = object.tag;
//         //               break;
//         //             case "TSNonNullExpression":
//         //               object = object.expression;
//         //               break;
//         //             default:
//         //               return false;
//         //           }
//         //         }
//     }
// }
