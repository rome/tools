use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::JsComputedMemberExpression;
use rome_js_syntax::JsComputedMemberExpressionFields;
use rome_rowan::AstNode;

impl FormatNodeFields<JsComputedMemberExpression> for FormatNodeRule<JsComputedMemberExpression> {
    fn fmt_fields(node: &JsComputedMemberExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let mut current = node.clone();

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

        write![
            f,
            [
                object.format(),
                group_elements(&format_args![
                    optional_chain_token.format(),
                    l_brack_token.format(),
                    soft_line_break(),
                    soft_block_indent(&member.format()),
                    r_brack_token.format()
                ]),
            ]
        ]?;

        // Traverse upwards again and concatenate the computed expression until we find the first non-computed expression
        while let Some(parent) = current
            .syntax()
            .parent()
            .and_then(JsComputedMemberExpression::cast)
        {
            // Don't traverse up if self is a member of a computed member expression
            if current == *node {
                break;
            }

            let JsComputedMemberExpressionFields {
                object: _,
                optional_chain_token,
                l_brack_token,
                member,
                r_brack_token,
            } = parent.as_fields();

            write!(
                f,
                [group_elements(&format_args![
                    optional_chain_token.format(),
                    l_brack_token.format(),
                    soft_line_break(),
                    soft_block_indent(&member.format()),
                    r_brack_token.format(),
                ])]
            )?;

            current = parent;
        }

        Ok(())
    }
}
