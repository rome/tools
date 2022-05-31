use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsTemplateLiteralType;
use rome_js_syntax::TsTemplateLiteralTypeFields;

impl FormatNodeFields<TsTemplateLiteralType> for FormatNodeRule<TsTemplateLiteralType> {
    fn format_fields(
        node: &TsTemplateLiteralType,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsTemplateLiteralTypeFields {
            l_tick_token,
            elements,
            r_tick_token,
        } = node.as_fields();

        formatted![
            formatter,
            [
                l_tick_token.format(),
                elements.format(),
                r_tick_token.format(),
            ]
        ]
    }
}
