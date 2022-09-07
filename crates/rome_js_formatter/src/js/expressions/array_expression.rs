use crate::prelude::*;

use crate::builders::format_delimited;
use crate::parentheses::NeedsParentheses;
use rome_formatter::write;
use rome_js_syntax::{
    JsAnyArrayElement, JsAnyExpression, JsArrayElementList, JsArrayExpressionFields,
};
use rome_js_syntax::{JsArrayExpression, JsSyntaxNode};
use rome_rowan::SyntaxResult;

#[derive(Debug, Clone, Default)]
pub struct FormatJsArrayExpression;

impl FormatNodeRule<JsArrayExpression> for FormatJsArrayExpression {
    fn fmt_fields(&self, node: &JsArrayExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsArrayExpressionFields {
            l_brack_token,
            elements,
            r_brack_token,
        } = node.as_fields();

        let r_brack_token = r_brack_token?;

        if elements.is_empty() {
            write!(
                f,
                [
                    l_brack_token.format(),
                    format_dangling_comments(node.syntax()).indented(),
                    r_brack_token.format(),
                ]
            )
        } else if should_break(&elements)? {
            write!(
                f,
                [
                    format_delimited(&l_brack_token?, &elements.format(), &r_brack_token)
                        .block_indent()
                ]
            )
        } else {
            let group_id = f.group_id("array");

            let elements = elements.format().with_options(Some(group_id));

            write!(
                f,
                [format_delimited(&l_brack_token?, &elements, &r_brack_token)
                    .soft_block_indent_with_group_id(Some(group_id))]
            )
        }
    }

    fn needs_parentheses(&self, item: &JsArrayExpression) -> bool {
        item.needs_parentheses()
    }
}

/// Returns `true` for arrays containing at least two elements if:
/// * all elements are either object or array expressions
/// * each child array expression has at least two elements, or each child object expression has at least two members.
fn should_break(elements: &JsArrayElementList) -> SyntaxResult<bool> {
    if elements.len() < 2 {
        Ok(false)
    } else {
        let mut elements = elements.iter().peekable();

        while let Some(element) = elements.next() {
            match element? {
                JsAnyArrayElement::JsAnyExpression(JsAnyExpression::JsArrayExpression(array)) => {
                    let next_is_array_or_end = matches!(
                        elements.peek(),
                        None | Some(Ok(JsAnyArrayElement::JsAnyExpression(
                            JsAnyExpression::JsArrayExpression(_)
                        )))
                    );
                    if array.elements().len() < 2 || !next_is_array_or_end {
                        return Ok(false);
                    }
                }
                JsAnyArrayElement::JsAnyExpression(JsAnyExpression::JsObjectExpression(object)) => {
                    let next_is_object_or_empty = matches!(
                        elements.peek(),
                        None | Some(Ok(JsAnyArrayElement::JsAnyExpression(
                            JsAnyExpression::JsObjectExpression(_)
                        )))
                    );

                    if object.members().len() < 2 || !next_is_object_or_empty {
                        return Ok(false);
                    }
                }
                _ => {
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }
}

impl NeedsParentheses for JsArrayExpression {
    #[inline(always)]
    fn needs_parentheses(&self) -> bool {
        false
    }
    #[inline(always)]
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
