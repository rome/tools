use crate::context::QuoteStyle;
use crate::jsx::auxiliary::space::JsxSpace;
use crate::prelude::*;
use crate::FormatElement;
use crate::FormatNodeFields;
use rome_formatter::{group_elements, FormatResult};
use rome_js_syntax::{
    JsAnyExpression, JsAnyLiteralExpression, JsxExpressionChild, JsxExpressionChildFields,
};

impl FormatNodeFields<JsxExpressionChild> for FormatNodeRule<JsxExpressionChild> {
    fn format_fields(
        node: &JsxExpressionChild,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsxExpressionChildFields {
            l_curly_token,
            expression,
            r_curly_token,
        } = node.as_fields();

        if let Some(JsAnyExpression::JsAnyLiteralExpression(
            JsAnyLiteralExpression::JsStringLiteralExpression(string_literal),
        )) = &expression
        {
            if let Ok(str_token) = string_literal.value_token() {
                let quote_style = formatter.context().quote_style();
                let space_str = match quote_style {
                    QuoteStyle::Double => "\" \"",
                    QuoteStyle::Single => "' '",
                };

                if str_token.text() == space_str {
                    let l_curly = formatter.format_replaced(&l_curly_token?, empty_element());
                    let space = formatter.format_replaced(&str_token, empty_element());
                    let r_curly = formatter.format_replaced(&r_curly_token?, empty_element());

                    return formatted![
                        formatter,
                        [
                            l_curly,
                            space,
                            JsxSpace::default().format(formatter),
                            r_curly
                        ]
                    ];
                }
            }
        }

        Ok(group_elements(formatted![
            formatter,
            [
                l_curly_token.format(),
                expression.format(),
                line_suffix_boundary(),
                r_curly_token.format()
            ]
        ]?))
    }
}
