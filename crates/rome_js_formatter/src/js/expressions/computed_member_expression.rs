use crate::format_traits::FormatOptional;
use rome_formatter::{
    concat_elements, group_elements, soft_block_indent, soft_line_break, FormatResult,
};

use crate::{format_elements, Format, FormatElement, FormatNode, Formatter};

use rome_js_syntax::JsComputedMemberExpression;
use rome_js_syntax::JsComputedMemberExpressionFields;
use rome_rowan::AstNode;

impl FormatNode for JsComputedMemberExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let mut current = self.clone();

        // Find the left most computed expression
        while let Some(computed_expression) =
            JsComputedMemberExpression::cast(current.object()?.syntax().clone())
        {
            current = computed_expression;
        }

        // Format the left most computed expression
        let JsComputedMemberExpressionFields {
            object,
            optional_chain_token,
            l_brack_token,
            member,
            r_brack_token,
        } = current.as_fields();

        let mut formatted = vec![format_elements![
            object.format(formatter)?,
            group_elements(format_elements![
                optional_chain_token.format_or_empty(formatter)?,
                l_brack_token.format(formatter)?,
                soft_line_break(),
                soft_block_indent(member.format(formatter)?),
                r_brack_token.format(formatter)?,
            ]),
        ]];

        // Traverse upwards again and concatenate the computed expression until we find the first non-computed expression
        while let Some(parent) = current
            .syntax()
            .parent()
            .and_then(JsComputedMemberExpression::cast)
        {
            // Don't traverse up if self is a member of a computed member expression
            if current == *self {
                break;
            }

            let JsComputedMemberExpressionFields {
                object: _,
                optional_chain_token,
                l_brack_token,
                member,
                r_brack_token,
            } = parent.as_fields();

            formatted.push(group_elements(format_elements![
                optional_chain_token.format_or_empty(formatter)?,
                l_brack_token.format(formatter)?,
                soft_line_break(),
                soft_block_indent(member.format(formatter)?),
                r_brack_token.format(formatter)?,
            ]));

            current = parent;
        }

        Ok(concat_elements(formatted))
    }
}
