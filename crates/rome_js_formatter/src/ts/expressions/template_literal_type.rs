use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::TsTemplateLiteralType;
use rome_js_syntax::TsTemplateLiteralTypeFields;

impl FormatNodeFields<TsTemplateLiteralType> for FormatNodeRule<TsTemplateLiteralType> {
    fn fmt_fields(node: &TsTemplateLiteralType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsTemplateLiteralTypeFields {
            l_tick_token,
            elements,
            r_tick_token,
        } = node.as_fields();

        write![
            f,
            [
                l_tick_token.format(),
                elements.format(),
                r_tick_token.format(),
            ]
        ]
    }
}
