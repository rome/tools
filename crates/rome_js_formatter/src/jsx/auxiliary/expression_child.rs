use crate::prelude::*;
use crate::prelude::{format_args, write};
use crate::utils::jsx::JsxSpace;
use rome_formatter::{group, CstFormatContext, FormatResult};
use rome_js_syntax::{
    JsAnyExpression, JsAnyLiteralExpression, JsxExpressionChild, JsxExpressionChildFields,
};

#[derive(Debug, Clone, Default)]
pub struct FormatJsxExpressionChild;

impl FormatNodeRule<JsxExpressionChild> for FormatJsxExpressionChild {
    fn fmt_fields(&self, node: &JsxExpressionChild, f: &mut JsFormatter) -> FormatResult<()> {
        let JsxExpressionChildFields {
            l_curly_token,
            expression,
            r_curly_token,
        } = node.as_fields();

        let l_curly_token = l_curly_token?;
        let r_curly_token = r_curly_token?;

        // If the expression child is just a string literal with one space in it, it's a JSX space
        if let Some(JsAnyExpression::JsAnyLiteralExpression(
            JsAnyLiteralExpression::JsStringLiteralExpression(string_literal),
        )) = &expression
        {
            let str_token = string_literal.value_token()?;
            let trimmed_text = str_token.text_trimmed();

            let has_space_text = trimmed_text == "' '" || trimmed_text == "\" \"";
            let no_trivia = !str_token.has_leading_non_whitespace_trivia()
                && !str_token.has_trailing_comments()
                && !l_curly_token.has_trailing_comments()
                && !r_curly_token.has_leading_non_whitespace_trivia();
            let is_suppressed = f
                .context()
                .comments()
                .is_suppressed(string_literal.syntax());

            if has_space_text && no_trivia && !is_suppressed {
                return write![
                    f,
                    [
                        format_removed(&l_curly_token),
                        format_replaced(&str_token, &JsxSpace::default()),
                        format_removed(&r_curly_token)
                    ]
                ];
            }
        }

        write![
            f,
            [group(&format_args![
                l_curly_token.format(),
                expression.format(),
                line_suffix_boundary(),
                r_curly_token.format()
            ])]
        ]
    }
}
