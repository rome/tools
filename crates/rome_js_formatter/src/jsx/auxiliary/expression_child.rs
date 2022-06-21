use crate::context::QuoteStyle;
use crate::jsx::auxiliary::space::JsxSpace;
use crate::prelude::*;
use crate::prelude::{format_args, write};
use crate::FormatNodeFields;
use rome_formatter::{group_elements, FormatResult};
use rome_js_syntax::{
    JsAnyExpression, JsAnyLiteralExpression, JsxExpressionChild, JsxExpressionChildFields,
};

impl FormatNodeFields<JsxExpressionChild> for FormatNodeRule<JsxExpressionChild> {
    fn fmt_fields(node: &JsxExpressionChild, f: &mut JsFormatter) -> FormatResult<()> {
        let JsxExpressionChildFields {
            l_curly_token,
            expression,
            r_curly_token,
        } = node.as_fields();

        // If the expression child is just a string literal with one space in it, it's a JSX space
        if let Some(JsAnyExpression::JsAnyLiteralExpression(
            JsAnyLiteralExpression::JsStringLiteralExpression(string_literal),
        )) = &expression
        {
            if let Ok(str_token) = string_literal.value_token() {
                let quote_style = f.context().quote_style();
                let space_str = match quote_style {
                    QuoteStyle::Double => "\" \"",
                    QuoteStyle::Single => "' '",
                };

                if str_token.text() == space_str {
                    let l_curly_token = l_curly_token?;
                    let r_curly_token = r_curly_token?;
                    let l_curly = format_removed(&l_curly_token);
                    let space = format_removed(&str_token);
                    let r_curly = format_removed(&r_curly_token);

                    return write![f, [l_curly, space, JsxSpace::default(), r_curly]];
                }
            }
        }

        write![
            f,
            [group_elements(&format_args![
                l_curly_token.format(),
                expression.format(),
                line_suffix_boundary(),
                r_curly_token.format()
            ])]
        ]
    }
}
