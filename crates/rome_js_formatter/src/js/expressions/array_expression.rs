use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use rome_formatter::{write, FormatRuleWithOptions};
use rome_js_syntax::{
    JsAnyArrayElement, JsAnyExpression, JsArrayElementList, JsArrayExpressionFields,
};
use rome_js_syntax::{JsArrayExpression, JsSyntaxNode};
use rome_rowan::SyntaxResult;

#[derive(Debug, Clone, Default)]
pub struct FormatJsArrayExpression {
    options: FormatJsArrayExpressionOptions,
}

impl FormatRuleWithOptions<JsArrayExpression> for FormatJsArrayExpression {
    type Options = FormatJsArrayExpressionOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.options = options;
        self
    }
}

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
                    format_dangling_comments(node.syntax()).with_block_indent(),
                    r_brack_token.format(),
                ]
            )
        } else {
            let group_id = f.group_id("array");

            let should_expand = !self.options.is_force_flat_mode && should_break(&elements)?;
            let elements = elements.format().with_options(Some(group_id));

            write!(
                f,
                [
                    l_brack_token.format(),
                    group(&soft_block_indent(&elements))
                        .with_group_id(Some(group_id))
                        .should_expand(should_expand),
                    r_brack_token.format()
                ]
            )
        }
    }

    fn needs_parentheses(&self, item: &JsArrayExpression) -> bool {
        item.needs_parentheses()
    }

    fn fmt_dangling_comments(
        &self,
        _: &JsArrayExpression,
        _: &mut JsFormatter,
    ) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct FormatJsArrayExpressionOptions {
    pub(crate) is_force_flat_mode: bool,
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
